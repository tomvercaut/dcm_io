use dicom_core::Tag;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    Attribute, DeriveInput, GenericArgument, LitStr, PathArguments, Type, TypePath,
    parse_macro_input,
};

#[derive(Default, Copy, Clone, Debug)]
struct AttrTag {
    group: u16,
    element: u16,
}

impl From<Tag> for AttrTag {
    fn from(tag: Tag) -> Self {
        Self {
            group: tag.0,
            element: tag.1,
        }
    }
}

impl From<AttrTag> for Tag {
    fn from(tag: AttrTag) -> Self {
        Tag(tag.group, tag.element)
    }
}

impl From<&AttrTag> for Tag {
    fn from(tag: &AttrTag) -> Self {
        Tag(tag.group, tag.element)
    }
}

#[derive(Default, Clone, Debug)]
struct DicomFieldAttr {
    pub tag: Option<AttrTag>,
    pub vr: Option<String>,
    transparent: bool,
}

fn parse_dicom_attr(attr: &Attribute) -> syn::Result<Option<DicomFieldAttr>> {
    if !attr.path().is_ident("dicom") {
        return Ok(None);
    }

    let mut dicom_attr = DicomFieldAttr::default();

    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("tag") {
            let value = meta.value()?;
            let lit: LitStr = value.parse()?;
            let tag_str = lit.value();

            // Parse tag format: "(0010,0020)"
            let tag_str = tag_str.trim_start_matches('(').trim_end_matches(')');
            let parts: Vec<&str> = tag_str.split(',').collect();

            if parts.len() != 2 {
                return Err(meta.error("Tag must be in format '(XXXX,XXXX)'"));
            }

            let group = u16::from_str_radix(parts[0], 16)
                .map_err(|_| meta.error("Invalid hexadecimal value for tag group"))?;
            let element = u16::from_str_radix(parts[1], 16)
                .map_err(|_| meta.error("Invalid hexadecimal value for tag element"))?;

            dicom_attr.tag = Some(AttrTag{group, element});
            Ok(())
        } else if meta.path.is_ident("vr") {
            let value = meta.value()?;
            let lit: LitStr = value.parse()?;
            dicom_attr.vr = Some(lit.value());
            Ok(())
        } else if meta.path.is_ident("transparent") {
            dicom_attr.transparent = true;
            Ok(())
        } else {
            Err(meta
                .error("Invalid dicom field attribute. Supported attributes are 'tag', 'vr' and 'transparent'"))
        }
    })?;
    if dicom_attr.transparent {
        return Ok(Some(DicomFieldAttr {
            tag: None,
            vr: None,
            transparent: true,
        }));
    }
    Ok(Some(dicom_attr))
}

fn get_inner_bracketed_type<'a, 'b>(ty: &'a Type, outer_ident_name: &'b str) -> Option<&'a Type> {
    let Type::Path(TypePath { path, .. }) = ty else {
        return None;
    };

    let segments = &path.segments;
    let last_segment = segments.last()?;
    let ident = &last_segment.ident;

    let ident_name = ident.to_string();
    if ident_name.as_str() != outer_ident_name {
        return None;
    }

    let PathArguments::AngleBracketed(generics) = &last_segment.arguments else {
        return None;
    };

    generics.args.first().and_then(|arg| {
        if let GenericArgument::Type(inner_type) = arg {
            Some(inner_type)
        } else {
            None
        }
    })
}

fn get_inner_type_option(ty: &Type) -> Option<&Type> {
    get_inner_bracketed_type(ty, "Option")
}

fn get_inner_type_vec(ty: &Type) -> Option<&Type> {
    get_inner_bracketed_type(ty, "Vec")
}

#[proc_macro_derive(Dicom, attributes(dicom))]
pub fn dicom_macro(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let ident = input.ident;

    let reader_ident = Ident::new(&format!("{}Reader", ident), ident.span());
    let fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &input.data {
        fields.iter().collect::<Vec<_>>()
    } else {
        vec![]
    };

    let mut qfields = vec![];
    let mut reading_fields = vec![];
    let mut self_fields = vec![];
    for field in fields {
        let mut dicom_field_attr = None;
        for attr in &field.attrs {
            if let Ok(Some(parsed_attr)) = parse_dicom_attr(attr) {
                if dicom_field_attr.is_some() {
                    return syn::Error::new_spanned(
                        attr,
                        "duplicate dicom attribute: only one dicom attribute is allowed per field",
                    )
                    .to_compile_error()
                    .into();
                }
                dicom_field_attr = Some(parsed_attr);
            }
        }
        if dicom_field_attr.is_none() {
            return syn::Error::new_spanned(field, "A dicom attribute is required.")
                .to_compile_error()
                .into();
        }
        let dicom_field_attr = dicom_field_attr.unwrap();
        if dicom_field_attr.transparent {
            continue;
        }

        let field_vis = field.vis.clone();
        let field_ident = field.ident.clone().unwrap();
        let field_ty = field.ty.clone();

        let inner_option_ty = get_inner_type_option(&field.ty);

        let lit_group = proc_macro2::Literal::u16_unsuffixed(dicom_field_attr.tag.as_ref().unwrap().group);
        let lit_element = proc_macro2::Literal::u16_unsuffixed(dicom_field_attr.tag.as_ref().unwrap().element);

        if inner_option_ty.is_none() {
            if dicom_field_attr.vr.as_ref().unwrap() == "SQ" {
                reading_fields.push(quote! {
                    let _ = read_option_seq(#field_ident);
                });
            } else {
                let inner_vec_ty = get_inner_type_vec(&field.ty);

                if inner_vec_ty.is_none() {
                    if let Some(vr) = dicom_field_attr.vr
                        && vr == "LO"
                    {
                        reading_fields.push(quote! {
                        let #field_ident = dcm_io::read_str(obj, dicom_core::Tag(#lit_group, #lit_element))?;
                    });
                        self_fields.push(quote! {
                            #field_ident: #field_ident,
                        });
                    }
                } else {
                    if let Some(vr) = dicom_field_attr.vr
                        && vr == "LO"
                    {
                        reading_fields.push(quote! {
                        let #field_ident = dcm_io::read_strs(obj, dicom_core::Tag(#lit_group, #lit_element))?;
                    });
                        self_fields.push(quote! {
                            #field_ident: #field_ident,
                        });
                    }
                }
            }
        } else {
            if dicom_field_attr.vr.as_ref().unwrap() == "SQ" {
                reading_fields.push(quote! {
                    let _ = read_option_value(#field_ident);
                });
            } else {

                reading_fields.push(quote! {
                    let _ = read_option_value(#field_ident);
                });
            }
        }

        let q = quote! {
            #field_vis #field_ident : #field_ty
        };
        qfields.push(q);
    }

    let expanded = quote! {
        #[derive(Clone, Debug, Default)]
        pub struct #reader_ident {
        }

        impl #reader_ident {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl dcm_io::DicomReader<#ident> for #reader_ident {
            fn read_dicom_obj(obj: &mut dicom_object::InMemDicomObject) -> dcm_io::Result<#ident> {
                #(#reading_fields)*
                Ok(
                    #ident {
                        #(#self_fields)*
                    }
                )
            }
        }
    };
    expanded.into()
}

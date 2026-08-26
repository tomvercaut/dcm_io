use dicom_core::{Tag, VR};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span};
use quote::quote;
use std::str::FromStr;
use syn::{
    Attribute, DeriveInput, Field, GenericArgument, LitStr, PathArguments, Type, TypePath,
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
    pub vr: Option<VR>,
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
            dicom_attr.vr = Some(VR::from_str(&lit.value()).map_err(|_| meta.error("Invalid VR value"))?);
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

fn literal_group_element(dicom_field_attr: &DicomFieldAttr) -> (Literal, Literal) {
    let lit_group = Literal::u16_unsuffixed(dicom_field_attr.tag.as_ref().unwrap().group);
    let lit_element = Literal::u16_unsuffixed(dicom_field_attr.tag.as_ref().unwrap().element);
    (lit_group, lit_element)
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
        let dicom_field_attr = match get_dicom_field_attr(&field) {
            Ok(value) => value,
            Err(value) => return value,
        };
        let dicom_field_attr = dicom_field_attr.unwrap();
        if dicom_field_attr.transparent {
            continue;
        }

        let field_vis = field.vis.clone();
        let field_ident = field.ident.clone().unwrap();
        let field_ty = field.ty.clone();

        handle_fields(
            &mut reading_fields,
            &mut self_fields,
            &dicom_field_attr,
            &field,
        );

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

fn get_dicom_field_attr(field: &&Field) -> Result<Option<DicomFieldAttr>, TokenStream> {
    let mut dicom_field_attr = None;
    for attr in &field.attrs {
        if let Ok(Some(parsed_attr)) = parse_dicom_attr(attr) {
            if dicom_field_attr.is_some() {
                return Err(syn::Error::new_spanned(
                    attr,
                    "duplicate dicom attribute: only one dicom attribute is allowed per field",
                )
                .to_compile_error()
                .into());
            }
            dicom_field_attr = Some(parsed_attr);
        }
    }
    if dicom_field_attr.is_none() {
        return Err(
            syn::Error::new_spanned(field, "A dicom attribute is required.")
                .to_compile_error()
                .into(),
        );
    }
    Ok(dicom_field_attr)
}

fn handle_fields(
    reading_fields: &mut Vec<proc_macro2::TokenStream>,
    self_fields: &mut Vec<proc_macro2::TokenStream>,
    dicom_field_attr: &DicomFieldAttr,
    field: &Field,
) {
    let vr = *dicom_field_attr.vr.as_ref().unwrap();
    let type_info = TypeInfo::new(&field.ty, vr);
    let (lit_group, lit_element) = literal_group_element(&dicom_field_attr);
    let field_ident = field.ident.clone().unwrap();
    match to_fn_name(vr, type_info.multiple, type_info.optional) {
        FnName::Seq => {
            // Handle sequences
            reading_fields.push(
                syn::Error::new_spanned(field, "Reading sequences is not yet implemented.")
                    .to_compile_error(),
            );
        }
        FnName::Unknown => {
            // Handle unknown
            let msg = format!(
                "Reading {} [optional: {}, multiple: {}] is not yet implemented.",
                vr, type_info.optional, type_info.multiple
            );
            reading_fields.push(syn::Error::new_spanned(field, msg).to_compile_error());
        }
        FnName::Name(fn_name) => {
            // handle the actual function name
            let fn_ident = Ident::new(&fn_name, Span::call_site());
            reading_fields.push(
                quote! {
                        let #field_ident = dcm_io::#fn_ident(obj, dicom_core::Tag(#lit_group, #lit_element))?;
                    });
            self_fields.push(quote! {
                #field_ident: #field_ident,
            });
        }
    }
}

struct TypeInfo {
    // Inner type of the field.
    // If the field is a sequence, this is the type of the sequence and not the inner type.
    ty: Type,
    // True if the field is a sequence.
    is_seq: bool,
    // Field can contain multiple values.
    multiple: bool,
    // Field is optional.
    optional: bool,
}

impl TypeInfo {
    /// Creates a new `TypeInfo` instance by analyzing a field's type and DICOM Value Representation.
    ///
    /// This method examines the Rust type to determine whether it represents an optional field
    /// (wrapped in `Option<T>`), a field with multiple values (wrapped in `Vec<T>`), or both.
    /// For sequence types (VR "SQ" or "sq"), it preserves the outer type structure.
    ///
    /// # Arguments
    ///
    /// * `ty` - The Rust type of the field to analyze (e.g., `String`, `Option<String>`, `Vec<String>`)
    /// * `vr` - The DICOM Value Representation (e.g., LO, SQ)
    ///
    /// # Returns
    ///
    /// Returns a `TypeInfo` struct containing:
    /// - `ty`: The inner type (or outer type for sequences)
    /// - `is_seq`: `true` if the VR is a sequence type ("SQ" or "sq")
    /// - `multiple`: `true` if the field can contain multiple values (`Vec<T>`)
    /// - `optional`: `true` if the field is optional (`Option<T>`)
    ///
    /// # Examples
    ///
    /// - `TypeInfo::new(&String, "LO")` → `{ ty: String, is_seq: false, multiple: false, optional: false }`
    /// - `TypeInfo::new(&Option<String>, "LO")` → `{ ty: String, is_seq: false, multiple: false, optional: true }`
    /// - `TypeInfo::new(&Vec<String>, "LO")` → `{ ty: String, is_seq: false, multiple: true, optional: false }`
    /// - `TypeInfo::new(&Option<Vec<String>>, "LO")` → `{ ty: String, is_seq: false, multiple: true, optional: true }`
    /// - `TypeInfo::new(&Vec<DicomObject>, "SQ")` → `{ ty: Vec<DicomObject>, is_seq: true, multiple: false, optional: false }`

    fn new(ty: &Type, vr: VR) -> Self {
        match get_inner_type_option(&ty) {
            None => match get_inner_type_vec(&ty) {
                None => Self {
                    ty: ty.clone(),
                    is_seq: false,
                    multiple: false,
                    optional: false,
                },
                Some(inner_vec_ty) => {
                    if vr == VR::SQ {
                        Self {
                            // Store the outer type of the sequence, not the inner.
                            ty: ty.clone(),
                            is_seq: true,
                            multiple: false,
                            optional: false,
                        }
                    } else {
                        Self {
                            ty: inner_vec_ty.clone(),
                            is_seq: false,
                            multiple: true,
                            optional: false,
                        }
                    }
                }
            },
            Some(inner_option_ty) => match get_inner_type_vec(&inner_option_ty) {
                None => Self {
                    ty: inner_option_ty.clone(),
                    is_seq: false,
                    multiple: false,
                    optional: true,
                },
                Some(inner_vec_ty) => {
                    if vr == VR::SQ {
                        Self {
                            // Store the outer type of the sequence, not the inner.
                            ty: inner_option_ty.clone(),
                            is_seq: true,
                            multiple: false,
                            optional: true,
                        }
                    } else {
                        Self {
                            ty: inner_vec_ty.clone(),
                            is_seq: false,
                            multiple: true,
                            optional: true,
                        }
                    }
                }
            },
        }
    }
}

#[derive(Clone, Debug)]
enum FnName {
    Seq,
    Unknown,
    Name(String),
}

/// Converts a DICOM Value Representation (VR) string into a function name identifier.
///
/// This function determines the appropriate reader function name based on the DICOM VR type
/// and the characteristics of the field (whether it contains multiple values and whether it's optional).
///
/// # Arguments
///
/// * `vr` - The DICOM Value Representation string (e.g., "LO", "SQ")
/// * `multiple` - Whether the field contains multiple values (e.g., `Vec<String>`)
/// * `optional` - Whether the field is optional (e.g., `Option<T>`)
///
/// # Returns
///
/// Returns an ` FnName ` enum variant:
/// - `FnName::Seq` for sequence (SQ) VR types
/// - `FnName::Name(String)` containing the generated function name (e.g., "read_str", "read_strs", "read_str_opt", "read_strs_opt")
/// - `FnName::Unknown` for unsupported VR types
///
/// # Examples
///
/// - `to_fn_name("LO", false, false)` returns `FnName::Name("read_str")`
/// - `to_fn_name("LO", true, false)` returns `FnName::Name("read_strs")`
/// - `to_fn_name("LO", false, true)` returns `FnName::Name("read_str_opt")`
/// - `to_fn_name("LO", true, true)` returns `FnName::Name("read_strs_opt")`
/// - `to_fn_name("SQ", _, _)` returns `FnName::Seq`
fn to_fn_name(vr: VR, multiple: bool, optional: bool) -> FnName {
    let fn_name = match vr {
        VR::SQ => FnName::Seq,
        VR::LO => FnName::Name("read_str".to_string()),
        _ => FnName::Unknown,
    };
    match fn_name {
        FnName::Seq => fn_name,
        FnName::Unknown => fn_name,
        FnName::Name(name) => {
            let mut tname = name;
            if multiple {
                tname.push_str("s");
            }
            if optional {
                tname.push_str("_opt");
            }
            FnName::Name(tname)
        }
    }
}

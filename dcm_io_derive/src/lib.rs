mod attr;
mod fn_name;
mod inner;
mod type_info;

use crate::attr::{DicomFieldAttr, get_dicom_field_attr};
use crate::fn_name::{FnName, to_fn_name};
use crate::type_info::TypeInfo;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span};
use quote::quote;
use syn::{DeriveInput, Field, parse_macro_input};

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
        #[derive(Clone, Debug)]
        pub struct #reader_ident {
        }

        impl Default for #reader_ident {
            fn default() -> Self {
                Self {
                }
            }
        }

        impl dcm_io::DicomReader<dicom_object::InMemDicomObject, #ident> for #reader_ident {
            fn read_dicom(backend: &dicom_object::InMemDicomObject) -> dcm_io::Result<#ident> {
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
                        let #field_ident = dcm_io::#fn_ident(backend, dicom_core::Tag(#lit_group, #lit_element))?;
                    });
            self_fields.push(quote! {
                #field_ident: #field_ident,
            });
        }
    }
}

use dicom_core::{Tag, VR};
use proc_macro::TokenStream;
use std::str::FromStr;
use syn::{Attribute, Field, LitStr};

#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct AttrTag {
    pub(crate) group: u16,
    pub(crate) element: u16,
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
pub(crate) struct DicomFieldAttr {
    pub tag: Option<AttrTag>,
    pub vr: Option<VR>,
    pub transparent: bool,
}

pub(crate) fn parse_dicom_attr(attr: &Attribute) -> syn::Result<Option<DicomFieldAttr>> {
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

            dicom_attr.tag = Some(AttrTag { group, element });
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

pub(crate) fn get_dicom_field_attr(field: &&Field) -> Result<Option<DicomFieldAttr>, TokenStream> {
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

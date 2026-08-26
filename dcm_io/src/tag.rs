use crate::Error;
use dicom_core::{value, Tag};
use dicom_core::value::Value;
use dicom_object::InMemDicomObject;
use dicom_object::mem::InMemElement;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DicomTag {
    pub group: u16,
    pub element: u16,
}

impl From<u32> for DicomTag {
    fn from(v: u32) -> Self {
        Self {
            group: (v >> 16) as u16,
            element: (v & 0xFFFF) as u16,
        }
    }
}

impl From<Tag> for DicomTag {
    fn from(tag: Tag) -> Self {
        Self {
            group: tag.0,
            element: tag.1,
        }
    }
}

impl DicomTag {
    pub fn new(group: u16, element: u16) -> Self {
        Self { group, element }
    }
}

pub fn read_tag(obj: &InMemDicomObject, tag: Tag) -> crate::Result<DicomTag> {
    let value = obj
        .element(tag)
        .map_err(|e| {
            eprintln!("{:#?}", e);
            Error::RequiredElementNotFound(tag.0, tag.1)
        })?;
    element_to_tag(value, tag)
}

pub fn read_tags(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<DicomTag>> {
    let value = obj
        .element(tag)
        .map_err(|_| Error::RequiredElementNotFound(tag.0, tag.1))?;
    element_to_tags(value, tag)
}

pub fn read_tag_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<DicomTag>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_tag(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

pub fn read_tags_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<Vec<DicomTag>>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_tags(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

fn element_to_tag(elem: &InMemElement, tag: Tag) -> Result<DicomTag, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::Tags(tags)) => {
            if tags.len() == 1 {
                let v = tags[0];
                let t = DicomTag::from(v);
                Ok(t)
            } else {
                if tags.len() == 0 {
                    Err(Error::MinimumRequiredElementsNotFound(tag.0, tag.1))
                } else {
                    Err(Error::TooManyRequiredElementsFound(tag.0, tag.1))
                }
            }
        }
        _ => {
            Err(Error::RequiredElementNotFound(tag.0, tag.1))
        }
    }
}

fn element_to_tags(elem: &InMemElement, tag: Tag) -> Result<Vec<DicomTag>, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::Tags(tags)) => {
            let v = tags.iter().map(|v| DicomTag::from(*v)).collect::<Vec<_>>();
            if v.is_empty() {
                Err(Error::MinimumRequiredElementsNotFound(tag.0, tag.1))
            } else {
                Ok(v)
            }
        }
        _ => {
            Err(Error::RequiredElementNotFound(tag.0, tag.1))
        }
    }
}

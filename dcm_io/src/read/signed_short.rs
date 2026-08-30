use crate::Error;
use dicom_core::value::Value;
use dicom_core::{Tag, value};
use dicom_object::InMemDicomObject;
use dicom_object::mem::InMemElement;

pub fn read_signed_short(obj: &InMemDicomObject, tag: Tag) -> crate::Result<i16> {
    let value = obj.element(tag).map_err(|e| {
        eprintln!("{:#?}", e);
        Error::RequiredElementNotFound(tag.0, tag.1)
    })?;
    element_to_i16(value, tag)
}

pub fn read_signed_shorts(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<i16>> {
    let value = obj
        .element(tag)
        .map_err(|_| Error::RequiredElementNotFound(tag.0, tag.1))?;
    element_to_i16s(value, tag)
}

pub fn read_signed_short_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<i16>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_i16(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

pub fn read_signed_shorts_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<Vec<i16>>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_i16s(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

fn element_to_i16(elem: &InMemElement, tag: Tag) -> Result<i16, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::I16(bytes)) => {
            if bytes.len() == 1 {
                let v = bytes[0];
                Ok(v)
            } else {
                if bytes.len() == 0 {
                    Err(Error::MinimumRequiredElementsNotFound(tag.0, tag.1))
                } else {
                    Err(Error::TooManyRequiredElementsFound(tag.0, tag.1))
                }
            }
        }
        _ => Err(Error::RequiredElementNotFound(tag.0, tag.1)),
    }
}

fn element_to_i16s(elem: &InMemElement, tag: Tag) -> Result<Vec<i16>, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::I16(bytes)) => {
            let v: Vec<i16> = bytes.to_vec();
            if v.is_empty() {
                Err(Error::MinimumRequiredElementsNotFound(tag.0, tag.1))
            } else {
                Ok(v)
            }
        }
        _ => Err(Error::RequiredElementNotFound(tag.0, tag.1)),
    }
}

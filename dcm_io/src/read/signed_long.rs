use crate::Error;
use dicom_core::value::Value;
use dicom_core::{Tag, value};
use dicom_object::InMemDicomObject;
use dicom_object::mem::InMemElement;

pub fn read_signed_long(obj: &InMemDicomObject, tag: Tag) -> crate::Result<i32> {
    let value = obj.element(tag).map_err(|e| {
        eprintln!("{:#?}", e);
        Error::RequiredElementNotFound(tag.0, tag.1)
    })?;
    element_to_i32(value, tag)
}

pub fn read_signed_longs(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<i32>> {
    let value = obj
        .element(tag)
        .map_err(|_| Error::RequiredElementNotFound(tag.0, tag.1))?;
    element_to_i32s(value, tag)
}

pub fn read_signed_long_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<i32>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_i32(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

pub fn read_signed_longs_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<Vec<i32>>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_i32s(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

fn element_to_i32(elem: &InMemElement, tag: Tag) -> Result<i32, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::I32(bytes)) => {
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

fn element_to_i32s(elem: &InMemElement, tag: Tag) -> Result<Vec<i32>, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::I32(bytes)) => {
            let v: Vec<i32> = bytes.to_vec();
            if v.is_empty() {
                Err(Error::MinimumRequiredElementsNotFound(tag.0, tag.1))
            } else {
                Ok(v)
            }
        }
        _ => Err(Error::RequiredElementNotFound(tag.0, tag.1)),
    }
}

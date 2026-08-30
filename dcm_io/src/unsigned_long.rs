use crate::Error;
use dicom_core::value::Value;
use dicom_core::{Tag, value};
use dicom_object::InMemDicomObject;
use dicom_object::mem::InMemElement;

pub fn read_unsigned_long(obj: &InMemDicomObject, tag: Tag) -> crate::Result<u32> {
    let value = obj.element(tag).map_err(|e| {
        eprintln!("{:#?}", e);
        Error::RequiredElementNotFound(tag.0, tag.1)
    })?;
    element_to_u32(value, tag)
}

pub fn read_unsigned_longs(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<u32>> {
    let value = obj
        .element(tag)
        .map_err(|_| Error::RequiredElementNotFound(tag.0, tag.1))?;
    element_to_u32s(value, tag)
}

pub fn read_unsigned_long_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<u32>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_u32(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

pub fn read_unsigned_longs_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<Vec<u32>>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_u32s(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

fn element_to_u32(elem: &InMemElement, tag: Tag) -> Result<u32, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::U32(bytes)) => {
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

fn element_to_u32s(elem: &InMemElement, tag: Tag) -> Result<Vec<u32>, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::U32(bytes)) => {
            let v: Vec<u32> = bytes.to_vec();
            if v.is_empty() {
                Err(Error::MinimumRequiredElementsNotFound(tag.0, tag.1))
            } else {
                Ok(v)
            }
        }
        _ => Err(Error::RequiredElementNotFound(tag.0, tag.1)),
    }
}

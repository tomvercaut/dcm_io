use crate::Error;
use dicom_core::value::Value;
use dicom_core::{Tag, value};
use dicom_object::InMemDicomObject;
use dicom_object::mem::InMemElement;

pub fn read_f32(obj: &InMemDicomObject, tag: Tag) -> crate::Result<f32> {
    let value = obj
        .element(tag)
        .map_err(|e| {
            eprintln!("{:#?}", e);
            Error::RequiredElementNotFound(tag.0, tag.1)
        })?;
    element_to_f32(value, tag)
}

pub fn read_f32s(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<f32>> {
    let value = obj
        .element(tag)
        .map_err(|_| Error::RequiredElementNotFound(tag.0, tag.1))?;
    element_to_f32s(value, tag)
}

pub fn read_f32_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<f32>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_f32(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

pub fn read_f32s_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<Vec<f32>>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_f32s(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

fn element_to_f32(elem: &InMemElement, tag: Tag) -> Result<f32, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::F32(floats)) => {
            if floats.len() == 1 {
                let v = floats[0];
                Ok(v)
            } else {
                if floats.len() == 0 {
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

fn element_to_f32s(elem: &InMemElement, tag: Tag) -> Result<Vec<f32>, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::F32(floats)) => {
            let v: Vec<f32> = floats.to_vec();
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

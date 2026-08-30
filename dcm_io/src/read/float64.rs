use crate::Error;
use dicom_core::value::Value;
use dicom_core::{Tag, value};
use dicom_object::InMemDicomObject;
use dicom_object::mem::InMemElement;

pub fn read_f64(obj: &InMemDicomObject, tag: Tag) -> crate::Result<f64> {
    let value = obj
        .element(tag)
        .map_err(|e| {
            eprintln!("{:#?}", e);
            Error::RequiredElementNotFound(tag.0, tag.1)
        })?;
    element_to_f64(value, tag)
}

pub fn read_f64s(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<f64>> {
    let value = obj
        .element(tag)
        .map_err(|_| Error::RequiredElementNotFound(tag.0, tag.1))?;
    element_to_f64s(value, tag)
}

pub fn read_f64_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<f64>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_f64(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

pub fn read_f64s_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<Vec<f64>>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_f64s(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

fn element_to_f64(elem: &InMemElement, tag: Tag) -> Result<f64, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::F64(floats)) => {
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

fn element_to_f64s(elem: &InMemElement, tag: Tag) -> Result<Vec<f64>, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::F64(floats)) => {
            let v: Vec<f64> = floats.to_vec();
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

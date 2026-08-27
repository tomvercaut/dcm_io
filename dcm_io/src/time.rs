use crate::Error;
use dicom_core::chrono::NaiveTime;
use dicom_core::value::Value;
use dicom_core::{Tag, value};
use dicom_object::InMemDicomObject;
use dicom_object::mem::InMemElement;

pub fn read_time(obj: &InMemDicomObject, tag: Tag) -> crate::Result<NaiveTime> {
    let value = obj
        .element(tag)
        .map_err(|e| {
            eprintln!("{:#?}", e);
            Error::RequiredElementNotFound(tag.0, tag.1)
        })?;
    element_to_time(value, tag)
}

pub fn read_times(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<NaiveTime>> {
    let value = obj
        .element(tag)
        .map_err(|_| Error::RequiredElementNotFound(tag.0, tag.1))?;
    element_to_times(value, tag)
}

pub fn read_time_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<NaiveTime>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_time(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

pub fn read_times_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<Vec<NaiveTime>>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_times(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

fn element_to_time(elem: &InMemElement, tag: Tag) -> Result<NaiveTime, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::Time(times)) => {
            if times.len() == 1 {
                let v = times[0];
                let t = v.to_naive_time()?;
                Ok(t)
            } else {
                if times.len() == 0 {
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

fn element_to_times(elem: &InMemElement, tag: Tag) -> Result<Vec<NaiveTime>, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::Time(times)) => {
            let v = times.iter().map(|v| v.to_naive_time()).collect::<Result<Vec<NaiveTime>, value::range::Error>>()?;
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

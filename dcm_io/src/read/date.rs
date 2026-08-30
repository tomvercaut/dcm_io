use crate::Error;
use dicom_core::chrono::NaiveDate;
use dicom_core::value::Value;
use dicom_core::{Tag, value};
use dicom_object::InMemDicomObject;
use dicom_object::mem::InMemElement;

pub fn read_date(obj: &InMemDicomObject, tag: Tag) -> crate::Result<NaiveDate> {
    let value = obj
        .element(tag)
        .map_err(|e| {
            eprintln!("{:#?}", e);
            Error::RequiredElementNotFound(tag.0, tag.1)
        })?;
    element_to_date(value, tag)
}

pub fn read_dates(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<NaiveDate>> {
    let value = obj
        .element(tag)
        .map_err(|_| Error::RequiredElementNotFound(tag.0, tag.1))?;
    element_to_dates(value, tag)
}

pub fn read_date_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<NaiveDate>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_date(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

pub fn read_dates_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<Vec<NaiveDate>>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_dates(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

fn element_to_date(elem: &InMemElement, tag: Tag) -> Result<NaiveDate, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::Date(dates)) => {
            if dates.len() == 1 {
                let v = dates[0];
                let t = v.to_naive_date()?;
                Ok(t)
            } else {
                if dates.len() == 0 {
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

fn element_to_dates(elem: &InMemElement, tag: Tag) -> Result<Vec<NaiveDate>, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::Date(dates)) => {
            let v = dates.iter().map(|v| v.to_naive_date()).collect::<Result<Vec<NaiveDate>, dicom_core::value::range::Error>>()?;
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

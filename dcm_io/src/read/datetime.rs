use crate::Error;
use dicom_core::chrono::{NaiveDateTime, NaiveTime};
use dicom_core::value::{DicomDateTime, Value};
use dicom_core::{Tag, value};
use dicom_object::InMemDicomObject;
use dicom_object::mem::InMemElement;

pub fn read_datetime(obj: &InMemDicomObject, tag: Tag) -> crate::Result<NaiveDateTime> {
    let value = obj
        .element(tag)
        .map_err(|e| {
            eprintln!("{:#?}", e);
            Error::RequiredElementNotFound(tag.0, tag.1)
        })?;
    element_to_date(value, tag)
}

pub fn read_datetimes(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<NaiveDateTime>> {
    let value = obj
        .element(tag)
        .map_err(|_| Error::RequiredElementNotFound(tag.0, tag.1))?;
    element_to_dates(value, tag)
}

pub fn read_datetime_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<NaiveDateTime>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_date(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

pub fn read_datetimes_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<Vec<NaiveDateTime>>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_dates(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

fn element_to_date(elem: &InMemElement, tag: Tag) -> Result<NaiveDateTime, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::DateTime(dates)) => {
            if dates.len() == 1 {
                to_naive_datetime(dates[0])
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

fn to_naive_datetime(ddt: DicomDateTime) -> Result<NaiveDateTime, Error> {
    let date = ddt.date().to_naive_date()?;
    let time = match ddt.time() {
        Some(time) => time.to_naive_time()?,
        None => NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    };
    Ok(NaiveDateTime::new(date, time))
}

fn element_to_dates(elem: &InMemElement, tag: Tag) -> Result<Vec<NaiveDateTime>, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::DateTime(dates)) => {
            let v = dates.iter().map(|v| to_naive_datetime(*v)).collect::<Result<Vec<NaiveDateTime>, Error>>()?;
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

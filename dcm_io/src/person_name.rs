use crate::Error;
use dicom_core::value::Value;
use dicom_core::{Tag, value};
use dicom_object::InMemDicomObject;
use dicom_object::mem::InMemElement;
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PersonName {
    pub last: Option<String>,
    pub first: Option<String>,
    pub middle: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

impl FromStr for PersonName {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('^');
        let last = match parts.next() {
            Some(name) => Some(name.to_string()),
            None => None,
        };
        let first = match parts.next() {
            Some(name) => Some(name.to_string()),
            None => None,
        };
        let middle = match parts.next() {
            Some(name) => Some(name.to_string()),
            None => None,
        };
        let prefix = match parts.next() {
            Some(name) => Some(name.to_string()),
            None => None,
        };
        let suffix = match parts.next() {
            Some(name) => Some(name.to_string()),
            None => None,
        };
        Ok(PersonName {
            last,
            first,
            middle,
            prefix,
            suffix,
        })
    }
}

impl std::fmt::Display for PersonName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 5;
        if self.suffix.is_none() {
            i -= 1;
            if self.prefix.is_none() {
                i -= 1;
                if self.middle.is_none() {
                    i -= 1;
                    if self.first.is_none() {
                        i -= 1;
                        if self.last.is_none() {
                            i -= 1;
                        }
                    }
                }
            }
        }
        match i {
            5 => write!(
                f,
                "{}^{}^{}^{}^{}",
                self.last.as_ref().unwrap_or(&"".to_string()),
                self.first.as_ref().unwrap_or(&"".to_string()),
                self.middle.as_ref().unwrap_or(&"".to_string()),
                self.prefix.as_ref().unwrap_or(&"".to_string()),
                self.suffix.as_ref().unwrap_or(&"".to_string())
            ),
            4 => write!(
                f,
                "{}^{}^{}^{}",
                self.last.as_ref().unwrap_or(&"".to_string()),
                self.first.as_ref().unwrap_or(&"".to_string()),
                self.middle.as_ref().unwrap_or(&"".to_string()),
                self.prefix.as_ref().unwrap_or(&"".to_string())
            ),
            3 => write!(
                f,
                "{}^{}^{}",
                self.last.as_ref().unwrap_or(&"".to_string()),
                self.first.as_ref().unwrap_or(&"".to_string()),
                self.middle.as_ref().unwrap_or(&"".to_string())
            ),
            2 => write!(
                f,
                "{}^{}",
                self.last.as_ref().unwrap_or(&"".to_string()),
                self.first.as_ref().unwrap_or(&"".to_string())
            ),
            1 => write!(f, "{}", self.last.as_ref().unwrap_or(&"".to_string())),
            _ => write!(f, ""),
        }
    }
}

pub fn read_person_name(obj: &InMemDicomObject, tag: Tag) -> crate::Result<PersonName> {
    let value = obj.element(tag).map_err(|e| {
        eprintln!("{:#?}", e);
        Error::RequiredElementNotFound(tag.0, tag.1)
    })?;
    element_to_person_name(value, tag)
}

pub fn read_person_names(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<PersonName>> {
    let value = obj
        .element(tag)
        .map_err(|_| Error::RequiredElementNotFound(tag.0, tag.1))?;
    element_to_person_names(value, tag)
}

pub fn read_person_name_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<PersonName>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_person_name(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

pub fn read_person_names_opt(
    obj: &InMemDicomObject,
    tag: Tag,
) -> crate::Result<Option<Vec<PersonName>>> {
    match obj.element_opt(tag)? {
        Some(elem) => {
            let dt = element_to_person_names(elem, tag)?;
            Ok(Some(dt))
        }
        None => Ok(None),
    }
}

fn element_to_person_name(elem: &InMemElement, tag: Tag) -> Result<PersonName, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::Strs(ints)) => {
            if ints.len() == 1 {
                let v = ints[0].parse::<PersonName>()?;
                Ok(v)
            } else {
                if ints.len() == 0 {
                    Err(Error::MinimumRequiredElementsNotFound(tag.0, tag.1))
                } else {
                    Err(Error::TooManyRequiredElementsFound(tag.0, tag.1))
                }
            }
        }
        _ => Err(Error::RequiredElementNotFound(tag.0, tag.1)),
    }
}

fn element_to_person_names(elem: &InMemElement, tag: Tag) -> Result<Vec<PersonName>, Error> {
    match elem.value() {
        Value::Primitive(value::PrimitiveValue::Strs(ints)) => {
            let v: Vec<PersonName> = ints
                .iter()
                .map(|s| s.parse::<PersonName>())
                .collect::<Result<Vec<PersonName>, _>>()?;
            if v.is_empty() {
                Err(Error::MinimumRequiredElementsNotFound(tag.0, tag.1))
            } else {
                Ok(v)
            }
        }
        _ => Err(Error::RequiredElementNotFound(tag.0, tag.1)),
    }
}

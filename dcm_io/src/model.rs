use std::str::FromStr;
use dicom_core::Tag;

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

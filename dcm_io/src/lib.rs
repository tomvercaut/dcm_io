use dicom_core::Tag;
use dicom_core::value::CastValueError;
use dicom_object::{AccessError, InMemDicomObject};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Required DICOM element not found: group {0}, element {1}")]
    RequiredElementNotFound(u16, u16),
    #[error("Failed to cast DICOM value.")]
    CastValueError(#[from] CastValueError),
    #[error("Failed to access DICOM value.")]
    AccessError(#[from] AccessError),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait DicomReader<T> {
    fn read_dicom_obj(obj: &mut InMemDicomObject) -> Result<T>;
}

pub trait DicomWriter<T> {
    fn write_dicom_obj(obj: &mut InMemDicomObject, model: &T) -> Result<()>;
}

#[derive(Debug, Clone)]
pub enum Value<T> {
    Single(T),
    Multiple(Vec<T>),
    Sequence(Vec<T>),
}

pub fn read_str(obj: &InMemDicomObject, tag: Tag) -> crate::Result<String> {
    let value = obj
        .element(tag)
        .map_err(|_| crate::Error::RequiredElementNotFound(tag.0, tag.1))?;
    let s = value
        .string()
        .map_err(|_| crate::Error::RequiredElementNotFound(tag.0, tag.1))?;
    Ok(s.to_string())
}

pub fn read_strs(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<String>> {
    let value = obj
        .element(tag)
        .map_err(|_| crate::Error::RequiredElementNotFound(tag.0, tag.1))?;
    let s = value
        .strings()
        .map_err(|_| crate::Error::RequiredElementNotFound(tag.0, tag.1))?;
    Ok(s.to_vec())
}

pub fn read_str_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<String>> {
    match obj.element_opt(tag)? {
        Some(elem) => Ok(Some(elem.string()?.to_string())),
        None => Ok(None)
    }
}

pub fn read_strs_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<Vec<String>>> {
    match obj.element_opt(tag)? {
        Some(elem) => Ok(Some(elem.strings()?.to_vec())),
        None => Ok(None)
    }
}
mod string;
mod tag;

use dicom_core::header::{ElementNumber, GroupNumber};
pub use string::*;
pub use tag::*;

use dicom_core::value::CastValueError;
use dicom_object::AccessError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Required DICOM element not found: group {0}, element {1}")]
    RequiredElementNotFound(u16, u16),
    #[error("Failed to cast DICOM value.")]
    CastValueError(#[from] CastValueError),
    #[error("Failed to access DICOM value.")]
    AccessError(#[from] AccessError),
    #[error("Required DICOM tag value not found: group {0}, element {1}")]
    RequiredDicomTagValueNotFound(u16, u16),
    #[error("Required DICOM tag values not found: group {0}, element {1}")]
    RequiredDicomTagValuesNotFound(u16, u16),
    #[error("Required DICOM tag minimum number of elements not found: group {0}, element {1}")]
    MinimumRequiredElementsNotFound(GroupNumber, ElementNumber),
    #[error("Found more DICOM tag elements: group {0}, element {1}")]
    TooManyRequiredElementsFound(GroupNumber, ElementNumber),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait DicomReader<Backend, T> {
    fn read_dicom(backend: &Backend) -> Result<T>;
}

pub trait DicomWriter<Backend, T> {
    fn write_dicom(backend: &mut Backend, model: &T) -> Result<()>;
}

#[derive(Debug, Clone)]
pub enum Value<T> {
    Single(T),
    Multiple(Vec<T>),
    Sequence(Vec<T>),
}
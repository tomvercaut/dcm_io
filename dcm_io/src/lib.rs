mod date;
mod datetime;
mod float32;
mod float64;
mod string;
mod tag;
mod time;
mod int_string;
mod other_byte;
mod other_long;
mod other_very_long;
mod other_word;
mod person_name;
mod signed_long;

pub use date::*;
pub use datetime::*;
use dicom_core::chrono;
use dicom_core::header::{ElementNumber, GroupNumber};
pub use float32::*;
pub use float64::*;
pub use string::*;
pub use tag::*;
pub use time::*;
pub use int_string::*;
pub use other_byte::*;
pub use other_long::*;
pub use other_very_long::*;
pub use other_word::*;
pub use person_name::*;
pub use signed_long::*;

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
    #[error("Failed to convert DICOM value to NaiveDate.")]
    InvalidDicomTagValue,
    #[error("Failed to parse date / time.")]
    ChroneParseError(#[from] chrono::ParseError),
    #[error("Failed to convert DICOM value to NaiveDate.")]
    DicomValueRangeError(#[from] dicom_core::value::range::Error),
    #[error("Failed to parse integer string.")]
    ParseIntError(#[from] std::num::ParseIntError),
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

use crate::inner::{get_inner_type_option, get_inner_type_vec};
use dicom_core::VR;
use syn::Type;

pub(crate) struct TypeInfo {
    // Inner type of the field.
    // If the field is a sequence, this is the type of the sequence and not the inner type.
    pub ty: Type,
    // True if the field is a sequence.
    pub is_seq: bool,
    // Field can contain multiple values.
    pub multiple: bool,
    // Field is optional.
    pub optional: bool,
}

impl TypeInfo {
    /// Creates a new `TypeInfo` instance by analyzing a field's type and DICOM Value Representation.
    ///
    /// This method examines the Rust type to determine whether it represents an optional field
    /// (wrapped in `Option<T>`), a field with multiple values (wrapped in `Vec<T>`), or both.
    /// For sequence types (VR "SQ" or "sq"), it preserves the outer type structure.
    ///
    /// # Arguments
    ///
    /// * `ty` - The Rust type of the field to analyze (e.g., `String`, `Option<String>`, `Vec<String>`)
    /// * `vr` - The DICOM Value Representation (e.g., LO, SQ)
    ///
    /// # Returns
    ///
    /// Returns a `TypeInfo` struct containing:
    /// - `ty`: The inner type (or outer type for sequences)
    /// - `is_seq`: `true` if the VR is a sequence type ("SQ" or "sq")
    /// - `multiple`: `true` if the field can contain multiple values (`Vec<T>`)
    /// - `optional`: `true` if the field is optional (`Option<T>`)
    ///
    /// # Examples
    ///
    /// - `TypeInfo::new(&String, "LO")` → `{ ty: String, is_seq: false, multiple: false, optional: false }`
    /// - `TypeInfo::new(&Option<String>, "LO")` → `{ ty: String, is_seq: false, multiple: false, optional: true }`
    /// - `TypeInfo::new(&Vec<String>, "LO")` → `{ ty: String, is_seq: false, multiple: true, optional: false }`
    /// - `TypeInfo::new(&Option<Vec<String>>, "LO")` → `{ ty: String, is_seq: false, multiple: true, optional: true }`
    /// - `TypeInfo::new(&Vec<DicomObject>, "SQ")` → `{ ty: Vec<DicomObject>, is_seq: true, multiple: false, optional: false }`

    pub(crate) fn new(ty: &Type, vr: VR) -> Self {
        match get_inner_type_option(&ty) {
            None => match get_inner_type_vec(&ty) {
                None => Self {
                    ty: ty.clone(),
                    is_seq: false,
                    multiple: false,
                    optional: false,
                },
                Some(inner_vec_ty) => {
                    if vr == VR::SQ {
                        Self {
                            // Store the outer type of the sequence, not the inner.
                            ty: ty.clone(),
                            is_seq: true,
                            multiple: false,
                            optional: false,
                        }
                    } else {
                        Self {
                            ty: inner_vec_ty.clone(),
                            is_seq: false,
                            multiple: true,
                            optional: false,
                        }
                    }
                }
            },
            Some(inner_option_ty) => match get_inner_type_vec(&inner_option_ty) {
                None => Self {
                    ty: inner_option_ty.clone(),
                    is_seq: false,
                    multiple: false,
                    optional: true,
                },
                Some(inner_vec_ty) => {
                    if vr == VR::SQ {
                        Self {
                            // Store the outer type of the sequence, not the inner.
                            ty: inner_option_ty.clone(),
                            is_seq: true,
                            multiple: false,
                            optional: true,
                        }
                    } else {
                        Self {
                            ty: inner_vec_ty.clone(),
                            is_seq: false,
                            multiple: true,
                            optional: true,
                        }
                    }
                }
            },
        }
    }
}

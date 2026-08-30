use dicom_core::VR;

#[derive(Clone, Debug)]
pub(crate) enum FnName {
    Seq,
    Unknown,
    Name(String),
}

/// Converts a DICOM Value Representation (VR) string into a function name identifier.
///
/// This function determines the appropriate reader function name based on the DICOM VR type
/// and the characteristics of the field (whether it contains multiple values and whether it's optional).
///
/// # Arguments
///
/// * `vr` - The DICOM Value Representation string (e.g., "LO", "SQ")
/// * `multiple` - Whether the field contains multiple values (e.g., `Vec<String>`)
/// * `optional` - Whether the field is optional (e.g., `Option<T>`)
///
/// # Returns
///
/// Returns an ` FnName ` enum variant:
/// - `FnName::Seq` for sequence (SQ) VR types
/// - `FnName::Name(String)` containing the generated function name (e.g., "read_str", "read_strs", "read_str_opt", "read_strs_opt")
/// - `FnName::Unknown` for unsupported VR types
///
/// # Examples
///
/// - `to_fn_name("LO", false, false)` returns `FnName::Name("read_str")`
/// - `to_fn_name("LO", true, false)` returns `FnName::Name("read_strs")`
/// - `to_fn_name("LO", false, true)` returns `FnName::Name("read_str_opt")`
/// - `to_fn_name("LO", true, true)` returns `FnName::Name("read_strs_opt")`
/// - `to_fn_name("SQ", _, _)` returns `FnName::Seq`
pub(crate) fn to_fn_name(vr: VR, multiple: bool, optional: bool) -> FnName {
    let fn_name = match vr {
        VR::SQ => FnName::Seq,
        VR::AE | VR::AS | VR::CS | VR::DS | VR::LO | VR::LT | VR::SH | VR::ST | VR::UC | VR::UI | VR::UT => {
            FnName::Name("read_str".to_string())
        }
        VR::AT => FnName::Name("read_tag".to_string()),
        VR::DA => FnName::Name("read_date".to_string()),
        VR::DT => FnName::Name("read_datetime".to_string()),
        VR::FL | VR::OF => FnName::Name("read_f32".to_string()),
        VR::FD | VR::OD => FnName::Name("read_f64".to_string()),
        VR::IS => FnName::Name("read_int_string".to_string()),
        VR::OB => FnName::Name("read_other_byte".to_string()),
        VR::OL => FnName::Name("read_other_long".to_string()),
        VR::OV => FnName::Name("read_other_very_long".to_string()),
        VR::OW => FnName::Name("read_other_word".to_string()),
        VR::PN => FnName::Name("read_person_name".to_string()),
        VR::SL => FnName::Name("read_signed_long".to_string()),
        VR::SS => FnName::Name("read_signed_short".to_string()),
        VR::SV => FnName::Name("read_signed_very_long".to_string()),
        VR::TM => FnName::Name("read_time".to_string()),
        _ => FnName::Unknown,
    };
    match fn_name {
        FnName::Seq => fn_name,
        FnName::Unknown => fn_name,
        FnName::Name(name) => {
            let mut tname = name;
            if multiple {
                tname.push_str("s");
            }
            if optional {
                tname.push_str("_opt");
            }
            FnName::Name(tname)
        }
    }
}

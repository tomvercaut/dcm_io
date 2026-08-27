#[cfg(test)]
mod tests {
    use dcm_io::{DicomReader};
    use dcm_io_derive::Dicom;
    use dicom_core::{DataElement, Tag, VR};

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredStrField {
        #[dicom(tag = "0010,0020", vr = "LO")]
        pub patient_id: String,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredStrsField {
        #[dicom(tag = "(0012,0063)", vr = "LO")]
        pub deidentification_method: Vec<String>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalStrField {
        #[dicom(tag = "0010,0020", vr = "LO")]
        pub patient_id: Option<String>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalStrsField {
        #[dicom(tag = "(0012,0063)", vr = "LO")]
        pub deidentification_method: Option<Vec<String>>,
    }

    #[test]
    fn read_required_str_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let _ = obj.put_str(Tag(0x0010, 0x0020), VR::LO, "123456");
        let required_field = RequiredStrFieldReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.patient_id.as_str(), "123456");
    }

    #[test]
    fn read_required_strs_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0012, 0x0063),
            VR::LO,
            dicom_core::PrimitiveValue::Strs(
                vec!["123456".to_string(), "789012".to_string()].into(),
            ),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredStrsFieldReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.deidentification_method.len(), 2);
        assert_eq!(
            required_field.deidentification_method[0].as_str(),
            "123456"
        );
        assert_eq!(
            required_field.deidentification_method[1].as_str(),
            "789012"
        );
    }

    #[test]
    fn read_optional_str_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let _ = obj.put_str(Tag(0x0010, 0x0020), VR::LO, "123456");
        let optional_field = OptionalStrFieldReader::read_dicom(&obj).unwrap();
        assert!(optional_field.patient_id.is_some());
        assert_eq!(
            optional_field.patient_id.as_ref().unwrap().as_str(),
            "123456"
        );
    }

    #[test]
    fn read_empty_optional_str_field() {
        let obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalStrFieldReader::read_dicom(&obj).unwrap();
        assert!(optional_field.patient_id.is_none());
    }

    #[test]
    fn read_optional_strs_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0012, 0x0063),
            VR::LO,
            dicom_core::PrimitiveValue::Strs(
                vec!["123456".to_string(), "789012".to_string()].into(),
            ),
        );
        let _ = obj.put_element(ime);
        let optional_field = OptionalStrsFieldReader::read_dicom(&obj).unwrap();
        assert!(optional_field.deidentification_method.is_some());
        assert_eq!(
            optional_field
                .deidentification_method
                .as_ref()
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            optional_field.deidentification_method.as_ref().unwrap()[0].as_str(),
            "123456"
        );
        assert_eq!(
            optional_field.deidentification_method.as_ref().unwrap()[1].as_str(),
            "789012"
        );
    }

    #[test]
    fn read_empty_optional_strs_field() {
        let obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalStrsFieldReader::read_dicom(&obj).unwrap();
        assert!(optional_field.deidentification_method.is_none());
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use dcm_io::{DicomReader};
    use dcm_io_derive::Dicom;
    use dicom_core::{DataElement, Tag, VR};

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredField {
        #[dicom(tag = "0010,0020", vr = "LO")]
        pub field: String,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredFields {
        #[dicom(tag = "(0012,0063)", vr = "LO")]
        pub field: Vec<String>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalField {
        #[dicom(tag = "0010,0020", vr = "LO")]
        pub field: Option<String>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalFields {
        #[dicom(tag = "(0012,0063)", vr = "LO")]
        pub field: Option<Vec<String>>,
    }

    #[test]
    fn read_required_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let _ = obj.put_str(Tag(0x0010, 0x0020), VR::LO, "123456");
        let required_field = RequiredFieldReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.field.as_str(), "123456");
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
        let required_field = RequiredFieldsReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.field.len(), 2);
        assert_eq!(
            required_field.field[0].as_str(),
            "123456"
        );
        assert_eq!(
            required_field.field[1].as_str(),
            "789012"
        );
    }

    #[test]
    fn read_optional_str_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let _ = obj.put_str(Tag(0x0010, 0x0020), VR::LO, "123456");
        let optional_field = OptionalFieldReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_some());
        assert_eq!(
            optional_field.field.as_ref().unwrap().as_str(),
            "123456"
        );
    }

    #[test]
    fn read_empty_optional_str_field() {
        let obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalFieldReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_none());
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
        let optional_field = OptionalFieldsReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_some());
        assert_eq!(
            optional_field
                .field
                .as_ref()
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            optional_field.field.as_ref().unwrap()[0].as_str(),
            "123456"
        );
        assert_eq!(
            optional_field.field.as_ref().unwrap()[1].as_str(),
            "789012"
        );
    }

    #[test]
    fn read_empty_optional_strs_field() {
        let obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalFieldsReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_none());
    }
}

fn main() {}

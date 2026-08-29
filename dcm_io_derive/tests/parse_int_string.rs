#[cfg(test)]
mod tests {
    use dcm_io::DicomReader;
    use dcm_io_derive::Dicom;
    use dicom_core::{DataElement, Tag, VR};

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredField {
        #[dicom(tag = "0010,0020", vr = "IS")]
        pub field: i32,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredFields {
        #[dicom(tag = "(0012,0063)", vr = "IS")]
        pub field: Vec<i32>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalField {
        #[dicom(tag = "0010,0020", vr = "IS")]
        pub field: Option<i32>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalFields {
        #[dicom(tag = "(0012,0063)", vr = "IS")]
        pub field: Option<Vec<i32>>,
    }

    #[test]
    fn read_required_dicom_tag_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::IS,
            dicom_core::PrimitiveValue::Strs(vec!["314".into()].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredFieldReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.field, 314);
    }

    #[test]
    fn read_required_dicom_tags_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0012, 0x0063),
            VR::IS,
            dicom_core::PrimitiveValue::Strs(vec![15.to_string(), 27.to_string()].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredFieldsReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.field.len(), 2);
        assert_eq!(required_field.field[0], 15);
        assert_eq!(required_field.field[1], 27);
    }

    #[test]
    fn read_optional_dicom_tag_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::IS,
            dicom_core::PrimitiveValue::Strs(vec![314.to_string()].into()),
        );
        let _ = obj.put_element(ime);
        let optionalfield = OptionalFieldReader::read_dicom(&obj).unwrap();
        assert!(optionalfield.field.is_some());
        assert_eq!(optionalfield.field.as_ref().unwrap(), &314);
    }

    #[test]
    fn read_empty_optional_dicom_tag_field() {
        let obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalFieldReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_none());
    }

    #[test]
    fn read_optional_dicom_tags_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0012, 0x0063),
            VR::IS,
            dicom_core::PrimitiveValue::Strs(vec![15.to_string(), 27.to_string()].into()),
        );
        let _ = obj.put_element(ime);
        let optional_field = OptionalFieldsReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_some());
        assert_eq!(optional_field.field.as_ref().unwrap().len(), 2);
        assert_eq!(optional_field.field.as_ref().unwrap()[0], 15);
        assert_eq!(optional_field.field.as_ref().unwrap()[1], 27);
    }

    #[test]
    fn read_empty_optional_dicom_tags_field() {
        let obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalFieldsReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_none());
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use dcm_io::{DicomReader, DicomTag};
    use dcm_io_derive::Dicom;
    use dicom_core::{DataElement, Tag, VR};

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredField {
        #[dicom(tag = "0010,0020", vr = "AT")]
        pub field: DicomTag,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredFields {
        #[dicom(tag = "(0012,0063)", vr = "AT")]
        pub field: Vec<DicomTag>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalField {
        #[dicom(tag = "0010,0020", vr = "AT")]
        pub field: Option<DicomTag>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalFields {
        #[dicom(tag = "(0012,0063)", vr = "AT")]
        pub field: Option<Vec<DicomTag>>,
    }

    #[test]
    fn read_required_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::AT,
            dicom_core::PrimitiveValue::Tags(vec![Tag(0x1234, 0x5600)].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredFieldReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.field, DicomTag::new(0x1234, 0x5600));
    }

    #[test]
    fn read_required_fields() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0012, 0x0063),
            VR::AT,
            dicom_core::PrimitiveValue::Tags(vec![Tag(0x1234, 0x5600), Tag(0x7890, 0x1200)].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredFieldsReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.field.len(), 2);
        assert_eq!(
            required_field.field[0],
            DicomTag::new(0x1234, 0x5600)
        );
        assert_eq!(
            required_field.field[1],
            DicomTag::new(0x7890, 0x1200)
        );
    }

    #[test]
    fn read_optional_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::AT,
            dicom_core::PrimitiveValue::Tags(vec![Tag(0x1234, 0x5600)].into()),
        );
        let _ = obj.put_element(ime);
        let optionalfield = OptionalFieldReader::read_dicom(&obj).unwrap();
        assert!(optionalfield.field.is_some());
        assert_eq!(
            optionalfield.field.as_ref().unwrap(),
            &DicomTag::new(0x1234, 0x5600)
        );
    }

    #[test]
    fn read_empty_optional_field() {
        let obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalFieldReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_none());
    }

    #[test]
    fn read_optional_fields() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0012, 0x0063),
            VR::AT,
            dicom_core::PrimitiveValue::Tags(vec![Tag(0x1234, 0x5600), Tag(0x7890, 0x1200)].into()),
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
            optional_field.field.as_ref().unwrap()[0],
            DicomTag::new(0x1234, 0x5600)
        );
        assert_eq!(
            optional_field.field.as_ref().unwrap()[1],
            DicomTag::new(0x7890, 0x1200)
        );
    }

    #[test]
    fn read_empty_optional_fields() {
        let obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalFieldsReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_none());
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use dcm_io::{DicomReader, DicomTag};
    use dcm_io_derive::Dicom;
    use dicom_core::{DataElement, Tag, VR};

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredDicomTagField {
        #[dicom(tag = "0010,0020", vr = "AT")]
        pub patient_id: DicomTag,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredDicomTagsField {
        #[dicom(tag = "(0012,0063)", vr = "AT")]
        pub deidentification_method: Vec<DicomTag>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalDicomTagField {
        #[dicom(tag = "0010,0020", vr = "AT")]
        pub patient_id: Option<DicomTag>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalDicomTagsField {
        #[dicom(tag = "(0012,0063)", vr = "AT")]
        pub deidentification_method: Option<Vec<DicomTag>>,
    }

    #[test]
    fn read_required_dicom_tag_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::AT,
            dicom_core::PrimitiveValue::Tags(vec![Tag(0x1234, 0x5600)].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredDicomTagFieldReader::read_dicom_obj(&mut obj).unwrap();
        assert_eq!(required_field.patient_id, DicomTag::new(0x1234, 0x5600));
    }

    #[test]
    fn read_required_dicom_tags_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0012, 0x0063),
            VR::AT,
            dicom_core::PrimitiveValue::Tags(vec![Tag(0x1234, 0x5600), Tag(0x7890, 0x1200)].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredDicomTagsFieldReader::read_dicom_obj(&mut obj).unwrap();
        assert_eq!(required_field.deidentification_method.len(), 2);
        assert_eq!(
            required_field.deidentification_method[0],
            DicomTag::new(0x1234, 0x5600)
        );
        assert_eq!(
            required_field.deidentification_method[1],
            DicomTag::new(0x7890, 0x1200)
        );
    }

    #[test]
    fn read_optional_dicom_tag_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::AT,
            dicom_core::PrimitiveValue::Tags(vec![Tag(0x1234, 0x5600)].into()),
        );
        let _ = obj.put_element(ime);
        let optionalfield = OptionalDicomTagFieldReader::read_dicom_obj(&mut obj).unwrap();
        assert!(optionalfield.patient_id.is_some());
        assert_eq!(
            optionalfield.patient_id.as_ref().unwrap(),
            &DicomTag::new(0x1234, 0x5600)
        );
    }

    #[test]
    fn read_empty_optional_dicom_tag_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalDicomTagFieldReader::read_dicom_obj(&mut obj).unwrap();
        assert!(optional_field.patient_id.is_none());
    }

    #[test]
    fn read_optional_dicom_tags_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0012, 0x0063),
            VR::AT,
            dicom_core::PrimitiveValue::Tags(vec![Tag(0x1234, 0x5600), Tag(0x7890, 0x1200)].into()),
        );
        let _ = obj.put_element(ime);
        let optional_field = OptionalDicomTagsFieldReader::read_dicom_obj(&mut obj).unwrap();
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
            optional_field.deidentification_method.as_ref().unwrap()[0],
            DicomTag::new(0x1234, 0x5600)
        );
        assert_eq!(
            optional_field.deidentification_method.as_ref().unwrap()[1],
            DicomTag::new(0x7890, 0x1200)
        );
    }

    #[test]
    fn read_empty_optional_dicom_tags_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalDicomTagsFieldReader::read_dicom_obj(&mut obj).unwrap();
        assert!(optional_field.deidentification_method.is_none());
    }
}

fn main() {}

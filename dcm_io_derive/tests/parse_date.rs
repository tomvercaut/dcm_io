#[cfg(test)]
mod tests {
    use dcm_io::DicomReader;
    use dcm_io_derive::Dicom;
    use dicom_core::{DataElement, Tag, VR};
    use dicom_core::chrono::NaiveDate;
    use dicom_core::value::DicomDate;

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredDicomTagField {
        #[dicom(tag = "0010,0020", vr = "DA")]
        pub field: NaiveDate,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredDicomTagsField {
        #[dicom(tag = "(0012,0063)", vr = "DA")]
        pub field: Vec<NaiveDate>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalDicomTagField {
        #[dicom(tag = "0010,0020", vr = "DA")]
        pub field: Option<NaiveDate>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalDicomTagsField {
        #[dicom(tag = "(0012,0063)", vr = "DA")]
        pub field: Option<Vec<NaiveDate>>,
    }

    #[test]
    fn read_required_dicom_tag_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::DA,
            dicom_core::PrimitiveValue::Date(vec![DicomDate::from_ymd(2023, 5, 15).unwrap()].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredDicomTagFieldReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.field, NaiveDate::from_ymd_opt(2023, 5, 15).unwrap());
    }

    #[test]
    fn read_required_dicom_tags_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0012, 0x0063),
            VR::DA,
            dicom_core::PrimitiveValue::Date(vec![DicomDate::from_ymd(2023, 5, 15).unwrap(), DicomDate::from_ymd(2024, 6, 20).unwrap()].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredDicomTagsFieldReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.field.len(), 2);
        assert_eq!(
            required_field.field[0],
            NaiveDate::from_ymd_opt(2023, 5, 15).unwrap()
        );
        assert_eq!(
            required_field.field[1],
            NaiveDate::from_ymd_opt(2024, 6, 20).unwrap()
        );
    }

    #[test]
    fn read_optional_dicom_tag_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::DA,
            dicom_core::PrimitiveValue::Date(vec![DicomDate::from_ymd(2023, 5, 15).unwrap()].into()),
        );
        let _ = obj.put_element(ime);
        let optionalfield = OptionalDicomTagFieldReader::read_dicom(&obj).unwrap();
        assert!(optionalfield.field.is_some());
        assert_eq!(
            optionalfield.field.as_ref().unwrap(),
            &NaiveDate::from_ymd_opt(2023, 5, 15).unwrap()
        );
    }

    #[test]
    fn read_empty_optional_dicom_tag_field() {
        let obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalDicomTagFieldReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_none());
    }

    #[test]
    fn read_optional_dicom_tags_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0012, 0x0063),
            VR::DA,
            dicom_core::PrimitiveValue::Date(vec![DicomDate::from_ymd(2023, 5, 15).unwrap(), DicomDate::from_ymd(2024, 6, 20).unwrap()].into()),
        );
        let _ = obj.put_element(ime);
        let optional_field = OptionalDicomTagsFieldReader::read_dicom(&obj).unwrap();
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
            NaiveDate::from_ymd_opt(2023, 5, 15).unwrap()
        );
        assert_eq!(
            optional_field.field.as_ref().unwrap()[1],
            NaiveDate::from_ymd_opt(2024, 6, 20).unwrap()
        );
    }

    #[test]
    fn read_empty_optional_dicom_tags_field() {
        let obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalDicomTagsFieldReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_none());
    }
}

fn main() {}

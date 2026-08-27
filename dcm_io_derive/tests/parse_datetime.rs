#[cfg(test)]
mod tests {
    use dcm_io::DicomReader;
    use dcm_io_derive::Dicom;
    use dicom_core::{DataElement, Tag, VR};
    use dicom_core::chrono::NaiveDateTime;
    use dicom_core::prelude::DicomDate;
    use dicom_core::value::{DicomDateTime, DicomTime};

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredField {
        #[dicom(tag = "0010,0020", vr = "DT")]
        pub field: NaiveDateTime,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct RequiredFields {
        #[dicom(tag = "(0012,0063)", vr = "DT")]
        pub field: Vec<NaiveDateTime>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalField {
        #[dicom(tag = "0010,0020", vr = "DT")]
        pub field: Option<NaiveDateTime>,
    }

    #[derive(Dicom, Default, Clone)]
    pub struct OptionalFields {
        #[dicom(tag = "(0012,0063)", vr = "DT")]
        pub field: Option<Vec<NaiveDateTime>>,
    }

    #[test]
    fn read_required_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::DT,
            dicom_core::PrimitiveValue::DateTime(vec![DicomDateTime::from_date_and_time(DicomDate::from_ymd(2024, 8, 27).unwrap(), DicomTime::from_hms(14, 30, 15).unwrap()).unwrap()].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredFieldReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.field, NaiveDateTime::parse_from_str("2024-08-27 14:30:15", "%Y-%m-%d %H:%M:%S").unwrap());
    }

    #[test]
    fn read_required_fields() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0012, 0x0063),
            VR::DT,
            dicom_core::PrimitiveValue::DateTime(vec![
                DicomDateTime::from_date_and_time(DicomDate::from_ymd(2024, 8, 27).unwrap(), DicomTime::from_hms(14, 30, 15).unwrap()).unwrap(),
                DicomDateTime::from_date_and_time(DicomDate::from_ymd(2024, 8, 27).unwrap(), DicomTime::from_hms(16, 45, 30).unwrap()).unwrap(),
            ].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredFieldsReader::read_dicom(&obj).unwrap();
        assert_eq!(required_field.field.len(), 2);
        assert_eq!(
            required_field.field[0],
            NaiveDateTime::parse_from_str("2024-08-27 14:30:15", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        assert_eq!(
            required_field.field[1],
            NaiveDateTime::parse_from_str("2024-08-27 16:45:30", "%Y-%m-%d %H:%M:%S").unwrap()
        );
    }

    #[test]
    fn read_optional_field() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::DT,
            dicom_core::PrimitiveValue::DateTime(vec![
                DicomDateTime::from_date_and_time(DicomDate::from_ymd(2024, 8, 27).unwrap(), DicomTime::from_hms(14, 30, 15).unwrap()).unwrap(),
            ].into()),
        );
        let _ = obj.put_element(ime);
        let optionalfield = OptionalFieldReader::read_dicom(&obj).unwrap();
        assert!(optionalfield.field.is_some());
        assert_eq!(
            optionalfield.field.as_ref().unwrap(),
            &NaiveDateTime::parse_from_str("2024-08-27 14:30:15", "%Y-%m-%d %H:%M:%S").unwrap()
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
            VR::DT,
            dicom_core::PrimitiveValue::DateTime(vec![
                DicomDateTime::from_date_and_time(DicomDate::from_ymd(2024, 8, 27).unwrap(), DicomTime::from_hms(14, 30, 15).unwrap()).unwrap(),
                DicomDateTime::from_date_and_time(DicomDate::from_ymd(2024, 8, 27).unwrap(), DicomTime::from_hms(16, 45, 30).unwrap()).unwrap(),
            ].into()),
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
            NaiveDateTime::parse_from_str("2024-08-27 14:30:15", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        assert_eq!(
            optional_field.field.as_ref().unwrap()[1],
            NaiveDateTime::parse_from_str("2024-08-27 16:45:30", "%Y-%m-%d %H:%M:%S").unwrap()
        );
    }

    #[test]
    fn read_empty_optional_fields() {
        let obj = dicom_object::InMemDicomObject::new_empty();
        let optional_field = OptionalFieldsReader::read_dicom(&obj).unwrap();
        assert!(optional_field.field.is_none());
    }

    #[test]
    fn read_datetime_with_microseconds() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::DT,
            dicom_core::PrimitiveValue::DateTime(vec![
                DicomDateTime::from_date_and_time(
                    DicomDate::from_ymd(2024, 8, 27).unwrap(),
                    DicomTime::from_hms_micro(14, 30, 15, 123_456).unwrap(),
                ).unwrap()
            ].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredFieldReader::read_dicom(&obj).unwrap();
        assert_eq!(
            required_field.field,
            NaiveDateTime::parse_from_str("2024-08-27 14:30:15.123456", "%Y-%m-%d %H:%M:%S%.f").unwrap()
        );
    }

    #[test]
    fn read_datetime_date_only() {
        let mut obj = dicom_object::InMemDicomObject::new_empty();
        let ime = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::DT,
            dicom_core::PrimitiveValue::DateTime(vec![
                DicomDateTime::from_date(DicomDate::from_ymd(2024, 8, 27).unwrap())
            ].into()),
        );
        let _ = obj.put_element(ime);
        let required_field = RequiredFieldReader::read_dicom(&obj).unwrap();
        assert_eq!(
            required_field.field,
            NaiveDateTime::parse_from_str("2024-08-27 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
    }
}

fn main() {}

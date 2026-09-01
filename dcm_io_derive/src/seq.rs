use dcm_io::{DicomReader, DicomSeqReader};
use dicom_core::value::Value;
use dicom_object::InMemDicomObject;
use dicom_object::mem::InMemElement;

pub struct DicomSequenceReader {}

impl DicomSeqReader<InMemElement> for DicomSequenceReader {
    fn read_dicom_seq<T, R>(backend: &InMemElement, _rdr: R) -> dcm_io::Result<Vec<T>>
    where
        R: DicomReader<InMemDicomObject, T>,
    {
        match backend.value() {
            Value::Primitive(_) => {
                Err(dcm_io::Error::ExpectedSequenceDicomElement)
            }
            Value::PixelSequence(_) => {
                Err(dcm_io::Error::ExpectedSequenceDicomElement)
            }
            Value::Sequence(seq) => {
                let mut v = Vec::new();
                for item in seq.items() {
                    let model = R::read_dicom(item)?;
                    v.push(model);
                }
                Ok(v)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicom_core::value::{DataSetSequence, PixelFragmentSequence, PrimitiveValue};
    use dicom_core::{DataElement, Tag, VR};

    struct TestItem {
        value: String,
    }

    struct TestItemReader;

    impl DicomReader<InMemDicomObject, TestItem> for TestItemReader {
        fn read_dicom(backend: &InMemDicomObject) -> dcm_io::Result<TestItem> {
            let elem = backend
                .element(Tag(0x0010, 0x0020))
                .map_err(|_| dcm_io::Error::RequiredElementNotFound(0x0010, 0x0020))?;
            let val = elem
                .string()
                .map_err(|_| dcm_io::Error::RequiredElementNotFound(0x0010, 0x0020))?;
            Ok(TestItem {
                value: val.to_string(),
            })
        }
    }

    #[test]
    fn test_read_sequence_success() {
        let mut item1 = InMemDicomObject::new_empty();
        item1.put_str(Tag(0x0010, 0x0020), VR::LO, "Item1");

        let mut item2 = InMemDicomObject::new_empty();
        item2.put_str(Tag(0x0010, 0x0020), VR::LO, "Item2");

        let seq = DataSetSequence::from(vec![item1, item2]);
        let element = DataElement::new(Tag(0x0008, 0x1115), VR::SQ, Value::Sequence(seq));

        let result = DicomSequenceReader::read_dicom_seq(&element, TestItemReader).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].value, "Item1");
        assert_eq!(result[1].value, "Item2");
    }

    #[test]
    fn test_read_empty_sequence() {
        let seq = DataSetSequence::empty();
        let element = DataElement::new(Tag(0x0008, 0x1115), VR::SQ, Value::Sequence(seq));

        let result = DicomSequenceReader::read_dicom_seq(&element, TestItemReader).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_read_primitive_error() {
        let element = DataElement::new(
            Tag(0x0010, 0x0020),
            VR::LO,
            Value::Primitive(PrimitiveValue::from("not a sequence")),
        );

        let result = DicomSequenceReader::read_dicom_seq(&element, TestItemReader);
        assert!(matches!(
            result,
            Err(dcm_io::Error::ExpectedSequenceDicomElement)
        ));
    }

    #[test]
    fn test_read_pixel_sequence_error() {
        let pixel_seq = PixelFragmentSequence::<Vec<u8>>::new_fragments(vec![]);
        let element = DataElement::new(
            Tag(0x7fe0, 0x0010),
            VR::OB,
            Value::PixelSequence(pixel_seq),
        );

        let result = DicomSequenceReader::read_dicom_seq(&element, TestItemReader);
        assert!(matches!(
            result,
            Err(dcm_io::Error::ExpectedSequenceDicomElement)
        ));
    }

    #[test]
    fn test_read_sequence_item_error() {
        let item1 = InMemDicomObject::new_empty(); // missing Tag(0x0010, 0x0020)
        let seq = DataSetSequence::from(vec![item1]);
        let element = DataElement::new(Tag(0x0008, 0x1115), VR::SQ, Value::Sequence(seq));

        let result = DicomSequenceReader::read_dicom_seq(&element, TestItemReader);
        assert!(matches!(
            result,
            Err(dcm_io::Error::RequiredElementNotFound(0x0010, 0x0020))
        ));
    }
}
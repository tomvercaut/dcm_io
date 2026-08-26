use dicom_core::Tag;
use dicom_object::InMemDicomObject;

pub fn read_str(obj: &InMemDicomObject, tag: Tag) -> crate::Result<String> {
    let value = obj
        .element(tag)
        .map_err(|_| crate::Error::RequiredElementNotFound(tag.0, tag.1))?;
    let s = value
        .string()
        .map_err(|_| crate::Error::RequiredElementNotFound(tag.0, tag.1))?;
    Ok(s.to_string())
}

pub fn read_strs(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Vec<String>> {
    let value = obj
        .element(tag)
        .map_err(|_| crate::Error::RequiredElementNotFound(tag.0, tag.1))?;
    let s = value
        .strings()
        .map_err(|_| crate::Error::RequiredElementNotFound(tag.0, tag.1))?;
    Ok(s.to_vec())
}

pub fn read_str_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<String>> {
    match obj.element_opt(tag)? {
        Some(elem) => Ok(Some(elem.string()?.to_string())),
        None => Ok(None)
    }
}

pub fn read_strs_opt(obj: &InMemDicomObject, tag: Tag) -> crate::Result<Option<Vec<String>>> {
    match obj.element_opt(tag)? {
        Some(elem) => Ok(Some(elem.strings()?.to_vec())),
        None => Ok(None)
    }
}
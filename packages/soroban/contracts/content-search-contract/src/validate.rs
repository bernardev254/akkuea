use soroban_sdk::String as SorobanString;

use crate::error::Error;
use crate::metadata::Content;

pub fn validate_subject(subject: &SorobanString) -> bool {
    !subject.is_empty()
}

pub fn validate_content(content: &Content) -> Result<(), Error> {
    if content.title.is_empty() || content.description.is_empty() || content.content_url.is_empty() {
        return Err(Error::InvalidInput);
    }
    if content.subject_tags.is_empty() {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

pub fn is_valid_tag(tag: &SorobanString) -> bool {
    !tag.is_empty()
} 
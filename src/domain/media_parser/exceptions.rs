use crate::domain::common::exceptions::DomainException;

use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
#[error("Age restriction `{raw_age_restriction}` parse error: {message}")]
pub struct AgeRestrictionParse<'a> {
    raw_age_restriction: Cow<'a, str>,
    message: Cow<'static, str>,
}

impl<'a> AgeRestrictionParse<'a> {
    pub fn new(
        raw_age_restriction: impl Into<Cow<'a, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            raw_age_restriction: raw_age_restriction.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Raw media type `{raw_media_type}` parse error: {message}")]
pub struct MediaTypeParse<'a> {
    raw_media_type: Cow<'a, str>,
    message: Cow<'static, str>,
}

impl<'a> MediaTypeParse<'a> {
    pub fn new(
        raw_media_type: impl Into<Cow<'a, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            raw_media_type: raw_media_type.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MediaParse<'a> {
    #[error(transparent)]
    AgeRestriction(AgeRestrictionParse<'a>),
    #[error(transparent)]
    MediaType(MediaTypeParse<'a>),
    #[error("No name provided")]
    NoNameProvided,
    #[error("No media type provided")]
    NoMediaTypeProvided,
    #[error("No age restriction provided")]
    NoAgeRestrictionProvided,
}

impl<'a> From<AgeRestrictionParse<'a>> for MediaParse<'a> {
    fn from(err: AgeRestrictionParse<'a>) -> Self {
        Self::AgeRestriction(err)
    }
}

impl<'a> From<MediaTypeParse<'a>> for MediaParse<'a> {
    fn from(err: MediaTypeParse<'a>) -> Self {
        Self::MediaType(err)
    }
}

impl DomainException for AgeRestrictionParse<'_> {}
impl DomainException for MediaTypeParse<'_> {}
impl DomainException for MediaParse<'_> {}

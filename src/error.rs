use crate::image::ImageKindVariants;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Operation ({0}) does not support given image kind {1}")]
    UnsupportedImageKind(&'static str, ImageKindVariants),
}

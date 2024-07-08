pub mod meta;
pub mod pyramid;

use crate::backend::{ImageBackend, PixelFormat};
use crate::error::Error;
use crate::math::KernelKind;
use core::fmt;
use meta::{ImageMetadata, Orientation, PatternFormat};
use std::fmt::write;
use std::marker::PhantomData;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageKindVariants {
    Bayer,
    Rgba,
    Gray,
}

impl fmt::Display for ImageKindVariants {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageKindVariants::Bayer => write!(f, "bayer"),
            ImageKindVariants::Rgba => write!(f, "rgba"),
            ImageKindVariants::Gray => write!(f, "gray"),
        }
    }
}

pub trait ImageKind {
    fn kind() -> ImageKindVariants;
}
pub struct BayerImage;
impl ImageKind for BayerImage {
    fn kind() -> ImageKindVariants {
        ImageKindVariants::Bayer
    }
}

pub struct RgbaImage;
impl ImageKind for RgbaImage {
    fn kind() -> ImageKindVariants {
        ImageKindVariants::Rgba
    }
}

pub struct GrayImage;
impl ImageKind for GrayImage {
    fn kind() -> ImageKindVariants {
        ImageKindVariants::Gray
    }
}

#[derive(Debug, Clone)]
pub struct Image<K: ImageKind, B: ImageBackend> {
    meta: Option<Arc<ImageMetadata>>,
    pixfmt: PixelFormat,
    pattern: PatternFormat,
    width: u32,
    height: u32,
    buffer: B::ImageBuffer,
    backend: Arc<B>,
    _k: PhantomData<K>,
}

impl<K: ImageKind, B: ImageBackend> Image<K, B> {
    pub fn new(
        backend: Arc<B>,
        width: u32,
        height: u32,
        pixfmt: PixelFormat,
        pattern: PatternFormat,
        meta: Option<Arc<ImageMetadata>>,
    ) -> Result<Self, Error> {
        let buffer =
            backend.create_image_buffer(pixfmt, pixfmt, width, height, pattern.channels())?;

        Ok(Self {
            meta,
            width,
            height,
            buffer,
            backend,
            pixfmt,
            pattern,
            _k: PhantomData,
        })
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.pattern.channels()
    }

    pub fn add(&self, dst: &mut Image<K, B>, other: &Image<K, B>) -> Result<(), Error> {
        Ok(())
    }

    pub fn div(&self, dst: &mut Image<K, B>, other: &Image<K, B>) -> Result<(), Error> {
        Ok(())
    }

    pub fn apply_orientation(&mut self, orientation: Orientation) -> Result<(), Error> {
        Ok(())
    }

    pub fn to_gray(&self, dst: &mut Image<GrayImage, B>, use_fft: bool) -> Result<(), Error> {
        Ok(())
    }

    pub fn upscale(&self, dst: &mut Image<K, B>) -> Result<(), Error> {}

    pub fn downsample(&self, dst: &mut Image<K, B>, factor: f32) -> Result<(), Error> {
        if !matches!(K::kind(), ImageKindVariants::Gray) {
            return Err(Error::UnsupportedImageKind("downsample", K::kind()));
        }

        self.backend
            .downsample(&mut dst.buffer, &self.buffer, KernelKind::Gaussian, factor)?;

        Ok(())
    }
}

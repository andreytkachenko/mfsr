pub mod cpu;

#[cfg(feature = "cuda")]
pub mod cuda;

#[cfg(feature = "wgpu")]
pub mod wgpu;

use std::sync::Arc;

use crate::{error::Error, math::KernelKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    HalfFloat,
    Float,
    Double,
    UnsignedByte,
    UnsignedShort,
    UnsignedInt,
    SignedByte,
    SignedShort,
    SignedInt,
}

pub trait Backend {
    type ImageBuffer;

    fn as_image_backend(
        self: Arc<Self>,
    ) -> Result<Arc<dyn ImageBackend<ImageBuffer = Self::ImageBuffer>>, Error>;
}

pub trait ImageBackend: Backend {
    fn create_image_buffer(
        &self,
        stored_px_fmt: PixelFormat,
        mapped_px_fmt: PixelFormat,
        width: u32,
        height: u32,
        channels: u32,
    ) -> Result<Self::ImageBuffer, Error>;

    fn load_image_data_int(&self, data: &[u16]);
    fn load_image_data_float(&self, data: &[f32]);

    ///
    ///    Apply a convolution by a kernel if required, then downsample an image.
    ///    Args:
    ///     	image: Device Array the input image (WARNING: single channel only!)
    ///     	kernel: None / str ('gaussian' / 'bayer') / 2d array
    ///     	factor: downsampling factor
    fn downsample(
        &self,
        dst: &mut Self::ImageBuffer,
        from: &Self::ImageBuffer,
        kernel: KernelKind,
        factor: f32,
    ) -> Result<(), Error>;
}

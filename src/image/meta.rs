use crate::{math::Mat4x3f, noise::NoiseModel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Normal,
    MirrorHoriz,
    Rotate180,
    MirrorVert,
    MirrorHorizAndRotate270Cw,
    Rotate90Cw,
    MirrorHorizAndRotate90Cw,
    Rotate270Cw,
}

#[derive(Debug, Clone)]
pub struct CfaPattern {}

#[derive(Debug, Clone)]
pub enum PatternFormat {
    Cfa(CfaPattern),
    Rgba,
}
impl PatternFormat {
    pub fn channels(&self) -> u32 {
        match self {
            PatternFormat::Cfa(..) => 1,
            PatternFormat::Rgba => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImageMetadata {
    /// Bayer (or XTrans or other) mosaic pattern kind
    pub cfa: CfaPattern,

    /// ISO value used to capture the image
    pub iso_speed: u32,

    /// Shutter speed used to capture image
    pub shutter_speed: f32,

    /// Time in milliseconds of exposure
    pub exposure_time: f32,

    /// Exposure bias value (e.g. -1EV)
    pub exposure_bias: i32,

    /// Aperture value of the image
    pub aperture_value: f32,

    /// Colour conversion matrix: xyz to rgb
    pub ccm: Mat4x3f,

    /// Image Noise Model
    pub noise_model: NoiseModel,
}

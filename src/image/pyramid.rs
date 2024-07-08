use std::sync::Arc;

use crate::{
    backend::{Backend, ImageBackend, PixelFormat},
    error::Error,
    image::Image,
    math::Rect,
};

use super::ImageKind;

pub struct ImagePyramid<B: ImageBackend> {
    img_rect: Rect<u32>,
    height: u32,
    width: u32,
    tile_size: u32,
    pyramid: Vec<B::ImageBuffer>,
    factors: Vec<f32>,
}

impl<B: ImageBackend> ImagePyramid<B> {
    ///
    ///     Returns the pyramid representation of ref_img, that will be used for
    ///     future block matching
    ///
    pub fn new(
        b: Arc<B>,
        width: u32,
        height: u32,
        tile_size: u32,
        factors: Vec<f32>,
    ) -> Result<Self, Error> {
        let img_back = b.clone().as_image_backend()?;

        let height_rem = height % tile_size;
        let width_rem = width % tile_size;

        let padding_patches_height = (tile_size - height_rem) * (height_rem != 0) as u32;
        let padding_patches_width = (tile_size - width_rem) * (width_rem != 0) as u32;

        let padded_width = width + padding_patches_width;
        let padded_height = height + padding_patches_height;

        let mut pyramid = Vec::with_capacity(factors.len());

        let (mut pw, mut ph) = (padded_width, padded_height);
        for f in factors.iter().copied() {
            let w = f32::floor(pw as f32 / f) as u32;
            let h = f32::floor(ph as f32 / f) as u32;

            pyramid.push(img_back.create_image_buffer(
                PixelFormat::Float,
                PixelFormat::Float,
                w,
                h,
                1,
            )?);

            (pw, ph) = (w, h);
        }

        Ok(Self {
            img_rect: Rect::new(0, 0, width, height),
            width: padded_width,
            height: padded_height,
            factors,
            tile_size,
            pyramid,
        })
    }

    pub fn load_image<K: ImageKind>(&mut self, img: &Image<K, B>) -> Result<(), Error> {
        // TODO!
        Ok(())
    }
}

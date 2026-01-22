//! Media processing module
//!
//! Image compression, resizing, thumbnail generation, and other media utilities.

pub mod image;

pub use image::{compress_image, generate_thumbnail, resize_image, ImageProcessingError};

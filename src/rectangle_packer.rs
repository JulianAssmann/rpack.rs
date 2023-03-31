use std::error::Error;
use std::fmt;
use crate::{Size, Area, Rectangle};

pub struct RectanglePackingResult {
    /// The list of rectangles that were packed.
    pub rectangles: Vec<Rectangle>,
    
    /// The size of the packed rectangle.
    pub size: Size,
}

impl RectanglePackingResult {
    /// Returns the packing ratio of the result.
    /// 
    /// The packing ratio is the ratio of the total area of the packed rectangles to the total area of the container rectangle.
    pub fn packing_ratio(&self) -> f64 {
        let total_area = self.size.area();
        let total_rect_area: usize = self.rectangles.iter().map(|r| r.area()).sum();
        total_rect_area as f64 / total_area as f64
    }
}

pub struct RectanglePackingError {
    /// The error message.
    pub message: String,

    /// The result of the packing operation until the error occurred.
    pub result: RectanglePackingResult,
}

impl fmt::Display for RectanglePackingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for RectanglePackingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RectanglePackingError")
            .field("message", &self.message)
            .field("result", &self.result)
            .finish()
    }
}

impl fmt::Debug for RectanglePackingResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RectanglePackingResult")
            .field("rectangles", &self.rectangles)
            .field("size", &self.size)
            .finish()
    }
}

impl Error for RectanglePackingError {}

pub struct RectanglePackerConfig {
    /// The maximum size of the container rectangle.
    /// 
    /// If `None`, the container dimensions will be dynamically determined to fit all the rectangles.
    /// If `Some`, the container will have the given dimensions, and an error will be returned if the rectangles cannot be packed within those dimensions.
    pub max_size: Option<Size>,

    /// The amount of padding to add around each rectangle.
    pub rectangle_padding: usize,

    /// The amount of padding to add around the container rectangle.
    pub border_padding: usize,
}

impl Default for RectanglePackerConfig {
    /// A default `RectanglePackerConfig` with the following values:
    /// - `max_size`: `None`
    /// - `rectangle_padding`: `0`
    /// - `border_padding`: `0`
    /// 
    /// # Returns
    /// A default `RectanglePackerConfig`.
    fn default() -> Self {
        RectanglePackerConfig {
            max_size: None,
            rectangle_padding: 0,
            border_padding: 0,
        }
    }
}

/// A trait for packing a list of rectangle sizes into a single container rectangle.
pub trait RectanglePacker {
    /// Packs a list of rectangle sizes into a single container rectangle.
    ///
    /// This method attempts to optimally arrange a list of rectangle sizes within a container rectangle
    /// such that the overall area or dimensions of the container are minimized, depending on the packing algorithm used.
    ///
    /// # Arguments
    /// * `elements` - A list of `Size` structs representing the dimensions of the rectangles to be packed.
    /// * `max_size` - An optional `Size` representing the maximum dimensions of the container rectangle.
    ///   - If `None`, the container dimensions will be dynamically determined to fit all the rectangles.
    ///   - If `Some`, the container will have the given dimensions, and an error will be returned if the rectangles cannot be packed within those dimensions.
    ///
    /// # Returns
    /// A `Result` containing either:
    ///   - A `RectanglePackingResult` with the list of packed `Rectangle`s and the dimensions of the container rectangle.
    ///   - A `RectanglePackingError` if the packing algorithm encounters an error or the provided `max_size` is insufficient to pack all the rectangles.
    fn pack(sizes: &Vec<Size>, config: &RectanglePackerConfig) -> Result<RectanglePackingResult, RectanglePackingError>;

    /// Checks that all the sizes can fit in the max size.
    /// 
    /// # Arguments
    /// * `sizes` - A list of `Size` structs representing the dimensions of the rectangles to be packed.
    /// * `max_size` - An optional `Size` representing the maximum dimensions of the container rectangle.
    /// 
    /// # Returns
    /// A `Result` containing either:
    ///   - `Ok(())` if all the sizes can fit in the max size.
    ///   - `Err(RectanglePackingError)` if any of the sizes are greater than the max size.
    fn check_sizes(sizes: &Vec<Size>, config: &RectanglePackerConfig) -> Result<(), RectanglePackingError> {
        let max_size = match config.max_size {
            Some(max_size) => Size {
                width: max_size.width - 2 * (config.border_padding + config.rectangle_padding), 
                height: max_size.height - 2 * (config.border_padding + config.rectangle_padding)
            },
            None => return Ok(())
        };
        for size in sizes {
            if size.width > max_size.width || 
               size.height > max_size.height {
                return Err(RectanglePackingError {
                    message: format!("Rectangle size {:?} is greater than max size {:?}", size, max_size),
                    result: RectanglePackingResult {
                        rectangles: Vec::new(),
                        size: Size::new(0, 0),
                    },
                });
            }
        }

        Ok(())
    }
}
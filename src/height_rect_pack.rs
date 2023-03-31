use std::cmp::max;

use crate::{RectanglePacker, Size, RectanglePackingError, RectanglePackingResult, Rectangle, RectanglePackerConfig};

/// A rectangle packer that packs rectangles by height.
pub struct HeightRectPacker {}

impl RectanglePacker for HeightRectPacker {
    fn pack(sizes: &Vec<Size>, config: &RectanglePackerConfig) -> Result<RectanglePackingResult, RectanglePackingError> {
        let max_size = match config.max_size {
            Some(max_size) => max_size,
            None => {
                // If no max size was specified, set the width of the container rectangle
                // to the average number of rectangles per row times the average width of a rectangle.
                // This is necessary because otherwise the algorithm would pack all the rectangles into a single row.
                // As the height of the container rectangle is not restricted, 
                // the algorithm will always be able to fit all rectangles.

                // Get the max width of all the rectangles
                let max_width = sizes.iter().map(|s| s.width).max().unwrap() + 2 * config.rectangle_padding;

                // Get the average width of all the rectangles
                let total_width: usize = sizes.iter().map(|s| s.width).sum();

                // Get the average number of rectangles per row
                let average_num_rectangles_per_row = (sizes.len() as f64 / total_width as f64).sqrt() as usize + 1;

                // Set the width to fit at least the largest rectangle in each row or the average number of rectangles per row
                let row_width = max(
                    (total_width / sizes.len()) * average_num_rectangles_per_row, 
                    max_width) + 2 * config.border_padding;
                    
                Size::new(row_width, usize::MAX)
            },
        };

        // Check that all sizes can fit in the max size
        Self::check_sizes(sizes, config)?;
        let mut rectangles = Vec::new();

        // Sort the sizes by height in descending order
        let mut sizes = sizes.clone();
        sizes.sort_unstable();

        // The current x and y positions for the left corner of the next rectangle
        let mut x: usize = config.border_padding + config.rectangle_padding;
        let mut y: usize = config.border_padding + config.rectangle_padding;

        // The largest height of the current row
        let mut largets_height: usize = 0;

        for size in sizes {
            
            // If adding the next rectangle would exceed the max width, move to the next row.
            // To do this, reset the x position to 0 and increment the y position by the 
            // largest height of any rectangle in the current row.
            if x + size.width + config.rectangle_padding > max_size.width - config.border_padding {
                x = 0;
                y += size.height + 2 * config.rectangle_padding;
                largets_height = 0;
            }

            // If adding the next rectangle would exceed the max height, return an error.
            if y + size.height + config.rectangle_padding > max_size.height - config.border_padding {
                return Err(RectanglePackingError {
                    message: format!("Could not fit all rectangles in max size"),
                    result: RectanglePackingResult {
                        rectangles: Vec::new(),
                        size: Size::new(x, y),
                    },
                });
            }

            // Update the x position in order to place the next rectangle to the right of the current one.
            x += size.width + 2 * config.rectangle_padding;

            // Update the largest height of the current row if necessary.
            if size.height > largets_height {
                largets_height = size.height;
            }

            // Add the rectangle to the list of packed rectangles.
            rectangles.push(Rectangle::from_size(x, y, &size));
        }

        Ok(RectanglePackingResult {
            rectangles: rectangles,
            size: Size::new(x, y + largets_height),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_pack() {
        // TODO add tests
    }
}
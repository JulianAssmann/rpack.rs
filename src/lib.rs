// Import the components
pub mod rectangle;
pub mod size;
pub mod rectangle_packer;
pub mod area;
pub mod height_rect_pack;

// Re-exports
pub use rectangle::Rectangle;
pub use size::Size;
pub use area::Area;
pub use crate::rectangle_packer::{RectanglePacker, RectanglePackingResult, RectanglePackingError, RectanglePackerConfig};
pub use height_rect_pack::HeightRectPacker;
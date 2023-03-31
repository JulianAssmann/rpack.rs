use crate::{Size, Area};

/// A rectangle in a 2D space.
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    /// The x coordinate of the rectangle.
    pub x: usize,

    /// The y coordinate of the rectangle.
    pub y: usize,

    /// The width of the rectangle.
    pub width: usize,

    /// The height of the rectangle.
    pub height: usize,
}

impl Rectangle {
    /// Creates a new rectangle.
    /// x and y are the coordinates of the top-left corner.
    /// width and height are the width and height of the rectangle.
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    /// Creates a new `Rectangle` instance with the given `x`, `y`, and `Size`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpack::{Rectangle, Size};
    /// 
    /// let size = Size::new(30, 40);
    /// let rect = Rectangle::from_size(10, 20, &size);
    /// ```
    pub fn from_size(x: usize, y: usize, size: &Size) -> Rectangle {
        Rectangle {
            x,
            y,
            width: size.width,
            height: size.height,
        }
    }

    /// Checks if the rectangle contains another rectangle.
    ///
    /// Returns `true` if the given `other` rectangle is completely inside the current rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpack::Rectangle;
    /// 
    /// let rect1 = Rectangle::new(10, 20, 30, 40);
    /// let rect2 = Rectangle::new(15, 25, 10, 10);
    /// let rect3 = Rectangle::new(40, 50, 10, 10);
    /// 
    /// assert!(rect1.contains(&rect2));
    /// assert!(!rect1.contains(&rect3));
    /// ```
    pub fn contains(&self, other: &Rectangle) -> bool {
        self.x <= other.x
            && self.y <= other.y
            && self.x + self.width >= other.x + other.width
            && self.y + self.height >= other.y + other.height
    }

    /// Checks if the rectangle intersects with another rectangle.
    ///
    /// Returns `true` if the given `other` rectangle intersects with the current rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpack::Rectangle;
    /// 
    /// let rect1 = Rectangle::new(10, 20, 30, 40);
    /// let rect2 = Rectangle::new(30, 40, 10, 10);
    /// let rect3 = Rectangle::new(40, 50, 10, 10);
    /// 
    /// assert!(rect1.intersects(&rect2));
    /// assert!(!rect1.intersects(&rect3));
    /// ```
    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Converts the `Rectangle` to a `Size` instance.
    ///
    /// This function returns a new `Size` instance with the same width and height as the `Rectangle`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpack::Rectangle;
    /// 
    /// let rect = Rectangle::new(10, 20, 30, 40);
    /// let size = rect.to_size();
    /// assert_eq!(size.width, 30);
    /// assert_eq!(size.height, 40);
    /// ```
    pub fn to_size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

impl Area for Rectangle {
    /// Returns the area of the rectangle.
    /// 
    /// # Examples
    /// ```
    /// use rpack::{Rectangle, Area};
    /// 
    /// let rect = Rectangle::new(2, 6, 3, 4);
    /// let area = rect.area(); // 12
    /// ```
    fn area(&self) -> usize {
        self.width * self.height
    }
}

impl PartialEq for Rectangle {
    /// Checks if two rectangles are equal.
    /// 
    /// # Examples
    /// ```
    /// use rpack::Rectangle;
    /// 
    /// let rect1 = Rectangle::new(2, 6, 3, 4);
    /// let rect2 = Rectangle::new(2, 6, 3, 4);
    /// let rect3 = Rectangle::new(2, 6, 3, 5);
    /// 
    /// assert!(rect1 == rect2);
    /// assert!(rect1 != rect3);
    /// ```
    fn eq(&self, other: &Rectangle) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.width == other.width
            && self.height == other.height
    }
}

impl Eq for Rectangle {}

#[cfg(test)]
mod tests {
    use crate::size::Size;

    use super::*;

    #[test]
    fn new_works() {
        let rect = Rectangle::new(3, 7, 14, 30);

        assert_eq!(rect.x, 3);
        assert_eq!(rect.y, 7);
        assert_eq!(rect.width, 14);
        assert_eq!(rect.height, 30);
    }

    #[test]
    fn from_size_works() {
        let size = Size::new(12, 24);
        let rect = Rectangle::from_size(2, 6, &size);

        assert_eq!(rect.x, 2);
        assert_eq!(rect.y, 6);
        assert_eq!(rect.width, 12);
        assert_eq!(rect.height, 24);
    }

    #[test]
    fn area_works() {
        let rect = Rectangle::new(3, 17, 5, 7);
        assert_eq!(rect.area(), 35);
    }

    #[test]
    fn intersection_works() {
        let a = Rectangle::new(0, 0, 5, 10);
        let b = Rectangle::new(2, 2, 10, 5);
        let c = Rectangle::new(10, 5, 1, 3);

        // a intersects b
        assert!(a.intersects(&b));
        assert!(b.intersects(&a));

        // b intersects c
        assert!(b.intersects(&c));
        assert!(c.intersects(&b));

        // a does not intersect c
        assert!(!a.intersects(&c));
        assert!(!c.intersects(&a));
    }

    #[test]
    fn contain_works() {
        let a = Rectangle::new(0, 0, 10, 10);
        let b = Rectangle::new(2, 2, 5, 5);
        let c = Rectangle::new(2, 2, 5, 10);

        assert!(a.contains(&b));
        assert!(!b.contains(&a));
        assert!(!a.contains(&c));
        assert!(!c.contains(&a));
        assert!(!b.contains(&c));
        assert!(c.contains(&b));
    }
}

use std::cmp::Ordering;

/// Represents a 2D size with width and height.
#[derive(Debug, Clone, Copy)]
pub struct Size {
    /// The width of the size.
    pub width: usize,

    /// The height of the size.
    pub height: usize,
}

impl Size {
    /// Creates a new `Size` instance with the given `width` and `height`.
    ///
    /// # Examples
    /// ```
    /// use rpack::Size;
    /// 
    /// let size = Size::new(10, 20);
    /// let width = size.width; // 10
    /// let height = size.height; // 20
    /// ```
    pub fn new(width: usize, height: usize) -> Size {
        Size { width, height }
    }

    /// Returns the area of the size.
    /// 
    /// # Examples
    /// ```
    /// use rpack::Size;
    /// 
    /// let size = Size::new(10, 20);
    /// let area = size.area(); // 200
    /// ```
    pub fn area(&self) -> usize {
        self.width * self.height
    }

    /// Returns `true` if the size is a square.
    /// 
    /// # Examples
    /// ```
    /// use rpack::Size;
    /// 
    /// let size = Size::new(10, 20);
    /// let is_square = size.is_square(); // false
    /// ```
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

impl PartialEq for Size {
    /// Returns true if the size is equal to another size.
    /// 
    /// # Examples
    /// ```
    /// use rpack::Size;
    /// 
    /// let size1 = Size::new(10, 20);
    /// let size2 = Size::new(10, 20);
    /// let size3 = Size::new(10, 30);
    /// 
    /// let is_equal = size1 == size2; // true
    /// let is_equal = size1 == size3; // false
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height && self.width == other.width
    }
}

impl Eq for Size {}

impl PartialOrd for Size {
    /// Returns the ordering of the size.
    /// The ordering is based on the height.
    /// If the height is equal, the ordering is based on the width.
    /// 
    /// # Examples
    /// ```
    /// use rpack::Size;
    /// 
    /// let size1 = Size::new(10, 20);
    /// let size2 = Size::new(10, 20);
    /// let size3 = Size::new(10, 30);
    /// let size4 = Size::new(20, 20);
    /// 
    /// let is_less = size1 < size2; // false
    /// let is_less = size1 < size3; // true
    /// let is_less = size3 < size4; // false
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Size {
    /// Returns the ordering of the size.
    /// 
    /// Returns the ordering of the size.
    /// The ordering is based on the height.
    /// If the height is equal, the ordering is based on the width.
    /// 
    /// # Examples
    /// ```
    /// use rpack::Size;
    /// 
    /// let size1 = Size::new(10, 20);
    /// let size2 = Size::new(10, 20);
    /// let size3 = Size::new(10, 30);
    /// let size4 = Size::new(20, 20);
    /// 
    /// let cmp = size1.cmp(&size2); // Equal
    /// let cmp = size1.cmp(&size3); // Less
    /// let cmp = size3.cmp(&size4); // Greater
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.height
            .cmp(&other.height)
            .then(self.width.cmp(&other.width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let size = Size::new(10, 20);
        assert_eq!(size.width, 10);
        assert_eq!(size.height, 20);

        let size = Size::new(3, 4);
        assert_eq!(size.width, 3);
        assert_eq!(size.height, 4);
    }

    #[test]
    fn area_works() {
        let size = Size::new(10, 20);
        assert_eq!(size.area(), 200);

        let size = Size::new(3, 4);
        assert_eq!(size.area(), 12);
    }

    #[test]
    fn is_square_works() {
        let size = Size::new(10, 20);
        assert_eq!(size.is_square(), false);

        let size = Size::new(3, 3);
        assert_eq!(size.is_square(), true);
    }

    #[test]
    fn eq_works() {
        let size1 = Size::new(10, 20);
        let size2 = Size::new(10, 20);
        assert_eq!(size1, size2);

        let size1 = Size::new(10, 20);
        let size2 = Size::new(10, 30);
        assert_ne!(size1, size2);

        let size1 = Size::new(10, 20);
        let size2 = Size::new(20, 20);
        assert_ne!(size1, size2);
    }

    #[test]
    fn ordering_works() {
        let size1 = Size::new(10, 20);
        let size2 = Size::new(10, 20);
        let size3 = Size::new(30, 20);
        let size4 = Size::new(30, 30);
        let size5 = Size::new(40, 40);

        assert_eq!(size1.cmp(&size2), Ordering::Equal);
        assert_eq!(size1.cmp(&size3), Ordering::Less);
        assert_eq!(size1.cmp(&size4), Ordering::Less);
        assert_eq!(size1.cmp(&size5), Ordering::Less);

        assert_eq!(size3.cmp(&size1), Ordering::Greater);
        assert_eq!(size3.cmp(&size2), Ordering::Greater);
        assert_eq!(size3.cmp(&size4), Ordering::Less);
        assert_eq!(size3.cmp(&size5), Ordering::Less);

        assert_eq!(size4.cmp(&size1), Ordering::Greater);
        assert_eq!(size4.cmp(&size2), Ordering::Greater);
        assert_eq!(size4.cmp(&size3), Ordering::Greater);
        assert_eq!(size4.cmp(&size5), Ordering::Less);

        assert_eq!(size5.cmp(&size1), Ordering::Greater);
        assert_eq!(size5.cmp(&size2), Ordering::Greater);
        assert_eq!(size5.cmp(&size3), Ordering::Greater);
        assert_eq!(size5.cmp(&size4), Ordering::Greater);
    }
}

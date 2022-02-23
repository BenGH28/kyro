#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub chpt: u32,
    pub verse: u32,
}

impl Point {
    pub fn new(ch: u32, vs: u32) -> Self {
        Self {
            chpt: ch,
            verse: vs,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.chpt == 0 && self.verse == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gt_point() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(1, 1);
        assert!(p1 == p2);

        let p1 = Point::new(2, 1);
        let p2 = Point::new(2, 2);
        assert!(p2 > p1);

        let p1 = Point::new(1, 1);
        let p2 = Point::new(2, 1);
        assert!(p2 > p1);
    }
    #[test]
    fn lt_point() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(1, 1);
        assert!(p1 == p2);

        let p1 = Point::new(2, 1);
        let p2 = Point::new(2, 2);
        assert!(p1 < p2);

        let p1 = Point::new(1, 1);
        let p2 = Point::new(2, 1);
        assert!(p1 < p2);
    }
}

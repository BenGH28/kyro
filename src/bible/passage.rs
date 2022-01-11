pub trait Passage {
    type Output;

    fn next(&mut self) -> Option<Self::Output>;
    fn previous(&mut self) -> Option<Self::Output>;
    fn at(&mut self) -> Option<Self::Output>;
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub chpt: u32,
    pub verse: u32,
}

impl Point {
    pub fn new() -> Self {
        Self { chpt: 0, verse: 0 }
    }
}

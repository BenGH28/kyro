pub trait Navigate {
    type Output;

    fn forward(&mut self) -> Option<&Self::Output>;

    fn backward(&mut self) -> Option<&Self::Output>;

    //get the enclosed content (i.e. for book: book.get(Point{1,1}) -> returns chapter 1)
    fn begin(&self) -> anyhow::Result<&Self::Output>;

    // there may not be a specified end point by the user
    fn end(&self) -> Option<&Self::Output>;
}

#[derive(Clone, Copy, Debug)]
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
        if self.chpt == 0 && self.verse == 0 {
            return true;
        }
        false
    }
}

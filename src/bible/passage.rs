pub trait Passage {
    type Output;

    fn next(&mut self) -> Option<Self::Output>;
    fn previous(&mut self) -> Option<Self::Output>;
    fn at(loc: EntryPoint) -> Option<Self::Output>;
}

pub struct EntryPoint {
    chpt: u32,
    verse: u32,
}

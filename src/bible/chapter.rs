use crate::bible::passage::Point;
use crate::bible::verse::Verse;

pub struct Chapter {
    pub number: u32,
    pub entry_point: Point,
    pub verses: Vec<Verse>,
}

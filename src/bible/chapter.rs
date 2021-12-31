use crate::bible::passage::EntryPoint;
use crate::bible::passage::Passage;
use crate::bible::verse::Verse;

pub struct Chapter {
    number: u32,
    entry_point: EntryPoint,
    verses: Vec<Verse>,
}

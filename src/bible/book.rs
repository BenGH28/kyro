use crate::bible::chapter::Chapter;
use crate::bible::passage::EntryPoint;
use crate::bible::passage::Passage;

pub struct Book {
    title: String,
    entry_point: EntryPoint,
    chapters: Vec<Chapter>,
}


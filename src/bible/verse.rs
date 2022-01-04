use super::passage::{Passage, EntryPoint};
use crate::parse;

pub struct Verse {
    pub number: u32,
    pub contents: String,
}

impl Passage for Verse {

    type Output = Verse;

    fn next(&mut self) -> Option<Self::Output> {
        todo!()
    }

    fn previous(&mut self) -> Option<Self::Output> {
        todo!()
    }

    ///get verse the user is looking for ie. Jn 3:16
    fn at(loc: EntryPoint) -> Option<Self::Output> {
        parse::parse(bible_string);
        todo!()
    }
}

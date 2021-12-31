
use crate::bible::chapter::Chapter;
use crate::bible::passage::Passage;
use crate::bible::passage::EntryPoint;

pub struct Book {
    title: String,
    entry_point: EntryPoint,
    chapters: Vec<Chapter>,
}

impl Passage for Book {
    type Output = Book;

    fn next(&mut self) -> Option<Book> {
        todo!()
    }
    fn previous(&mut self) -> Option<Book> {
        todo!()
    }

    fn at(location: EntryPoint) -> Option<Book> {
        todo!()
    }
}

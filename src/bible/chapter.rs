use crate::bible::paragraph::Paragraph;
use crate::bible::passage::Point;

#[derive(Debug)]
pub struct Chapter {
    pub number: u32,
    pub entry_point: Point,
    pub paragraphs: Vec<Paragraph>,
}

use crate::bible::passage::Point;
use anyhow::Context;

#[derive(Debug, Clone, Copy)]
pub struct Query {
    pub entry_point: Point,
    pub end_point: Point,
    pub vs_next: u32,
    pub chpt_next: u32,
}

impl Query {
    pub fn new(entry_point: Point, end_point: Point) -> Self {
        Self {
            entry_point,
            end_point,
            vs_next: 0,
            chpt_next: 0,
        }
    }

    pub fn is_range(&self) -> bool {
        self.entry_point < self.end_point
    }

    ///checks if the passage spans inside the chapter
    pub fn is_internal_range(&self) -> bool {
        self.entry_point.chpt == self.end_point.chpt
            && self.entry_point.verse != self.end_point.verse
    }

    ///saved for later
    pub fn next_chpt(&mut self) {
        if self.chpt_next == 0 {
            self.chpt_next = self.entry_point.chpt + 1;
        } else {
            self.chpt_next += 1;
        }
    }

    ///saved for later
    pub fn next_vs(&mut self) {
        if self.vs_next == 0 {
            self.vs_next = self.entry_point.verse + 1;
        } else {
            self.vs_next += 1;
        }
    }

    pub fn setup_query(chapter_verse: String) -> anyhow::Result<Query> {
        let (start_point, end_point) = Query::split_cli_query(&chapter_verse)?;
        let query = Query::new(start_point, end_point);
        Ok(query)
    }

    pub fn get_query_start(start_vec: &[&str]) -> anyhow::Result<Point> {
        let start_chpt = start_vec[0]
            .parse::<u32>()
            .context("starting chapter invalid")?;

        let start_vs: u32;
        if start_vec.len() > 1 {
            start_vs = start_vec[1]
                .parse::<u32>()
                .context("starting verse is invalid")?;
        } else {
            start_vs = 1;
        }

        let start = Point {
            chpt: start_chpt,
            verse: start_vs,
        };

        Ok(start)
    }

    pub fn split_cli_query(chpt_vs_query: &str) -> anyhow::Result<(Point, Point)> {
        let dash = '-';
        let colon = ':';

        // if the user just enters the book title and no chapter:verse then start them off at the
        // beginning of the book
        if chpt_vs_query.is_empty() {
            return Ok((Point::new(1, 1), Point::new(1, 0)));
        }

        let split_on_dash: Vec<&str> = chpt_vs_query.split(dash).collect();

        let start_vec: Vec<&str> = split_on_dash[0].split(colon).collect();
        let start = Query::get_query_start(&start_vec)?;

        let end: Point;
        if split_on_dash.len() == 1 {
            end = Point::new(0, 0);
        } else {
            //WARNING:this only exists if there is a dash
            let end_vec: Vec<&str> = split_on_dash[1].split(colon).collect();

            let has_chpt = end_vec.len() == 2;
            let end_chpt = if has_chpt {
                end_vec[0]
                    .parse::<u32>()
                    .context("ending chapter is invalid")?
            } else {
                start.chpt
            };

            let end_vs = if has_chpt {
                end_vec[1]
                    .parse::<u32>()
                    .context("ending verse is invalid")?
            } else {
                end_vec[0]
                    .parse::<u32>()
                    .context("ending verse is invalid")?
            };

            end = Point {
                chpt: end_chpt,
                verse: end_vs,
            };
        }

        Ok((start, end))
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn query_fixture() -> Query {
        let start = Point::new(9, 3);
        let end = Point::new(10, 8);
        Query::new(start, end)
    }

    #[rstest]
    fn query_next_chpt(mut query_fixture: Query) {
        query_fixture.next_chpt();
        let expected = 10;
        let actual = query_fixture.chpt_next;
        assert_eq!(expected, actual);
    }

    #[rstest]
    fn query_next_vs(mut query_fixture: Query) {
        query_fixture.next_vs();
        let expected = 4;
        let actual = query_fixture.vs_next;
        assert_eq!(expected, actual);
    }
}

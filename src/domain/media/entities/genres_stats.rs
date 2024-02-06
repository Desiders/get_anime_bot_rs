use super::GenreStats;

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenresStats(pub Vec<GenreStats>);

impl From<Vec<GenreStats>> for GenresStats {
    fn from(genres: Vec<GenreStats>) -> Self {
        Self(genres)
    }
}

impl Display for GenresStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for genre in &self.0 {
            writeln!(f, "{genre}")?;
        }
        Ok(())
    }
}

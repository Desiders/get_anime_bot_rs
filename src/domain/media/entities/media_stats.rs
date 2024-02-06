use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaStats {
    pub total: i64,
    pub gif: i64,
    pub image: i64,
    pub unknown: i64,
    pub sfw: i64,
    pub nsfw: i64,
}

impl Display for MediaStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Total: {}\nGIF: {}\nImage: {}\nUnknown: {}\nSFW: {}\nNSFW: {}",
            self.total, self.gif, self.image, self.unknown, self.sfw, self.nsfw
        )
    }
}

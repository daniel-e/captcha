use base64::{prelude::BASE64_STANDARD, Engine};
use std::collections::HashMap;

pub trait Font {
    fn png_as_base64(&self, letter: char) -> Option<&str>;

    fn chars(&self) -> Vec<char>;

    /// Returns None if letter does not exist or if letter could not decoded.
    fn png(&self, letter: char) -> Option<Vec<u8>> {
        self.png_as_base64(letter)
            .map(|v| BASE64_STANDARD.decode(v).ok())?
    }
}

pub struct Default {
    data: HashMap<char, String>,
}

impl Default {
    pub fn new() -> Self {
        let json = include_str!("font_default.json");

        Self {
            data: serde_json::from_str(json).expect("invalid json"),
        }
    }
}

impl Font for Default {
    fn png_as_base64(&self, letter: char) -> Option<&str> {
        self.data.get(&letter).map(|v| v.as_str())
    }

    fn chars(&self) -> Vec<char> {
        self.data.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::fonts::{Default, Font};
    use crate::images::Image;

    #[test]
    fn fonts_default() {
        let f = Default::new();

        assert_eq!(f.chars().len(), 57);
        assert!(f.png_as_base64('a').is_some());
        assert!(f.png('a').is_some());
        for i in f.chars() {
            assert!(Image::from_png(f.png(i).unwrap()).is_some());
        }
    }
}

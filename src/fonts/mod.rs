use serde_json;
use std::collections::HashMap;
use base64::decode;

pub trait Font {
    fn png_as_base64(&self, letter: String) -> Option<&String>;

    fn chars(&self) -> Vec<String>;

    /// Returns None if letter does not exist or if letter could not decoded.
    fn png(&self, letter: String) -> Option<Vec<u8>> {
        match self.png_as_base64(letter) {
            None    => None,
            Some(s) => {
                match decode(s) {
                    Err(_) => None,
                    Ok(v)  => Some(v)
                }
            }
        }
    }
}

pub struct Default {
    data: HashMap<String, String>
}

impl Default {
    pub fn new() -> Default {
        let json = include_str!("font_default.json").to_string();
        let d: HashMap<String, String> = serde_json::from_str(&json).expect("invalid json");

        Default {
            data: d
        }
    }
}

impl Font for Default {
    fn png_as_base64(&self, letter: String) -> Option<&String> {
        self.data.get(&letter)
    }

    fn chars(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use images::Image;
    use fonts::{Default, Font};

    #[test]
    fn fonts_default() {
        let f = Default::new();

        assert_eq!(f.chars().len(), 57);
        assert!(f.png_as_base64("a".to_string()).is_some());
        assert!(f.png("a".to_string()).is_some());
        for i in f.chars() {
            assert!(Image::from_png(f.png(i).unwrap()).is_some());
        }
    }
}

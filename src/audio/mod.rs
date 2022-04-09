#[cfg(feature = "audio")]
use base64::decode;
#[cfg(feature = "audio")]
use hound::Result;
#[cfg(feature = "audio")]
use rand::{thread_rng, Rng};
#[cfg(feature = "audio")]
use serde_json;
#[cfg(feature = "audio")]
use std::collections::HashMap;
#[cfg(feature = "audio")]
use std::io::Cursor;

#[cfg(feature = "audio")]
pub struct Audio {
    data: HashMap<char, String>,
}

#[cfg(feature = "audio")]
impl Audio {
    pub fn new() -> Audio {
        let json = include_str!("audio.json").to_string();

        Audio {
            data: serde_json::from_str(&json).expect("invalid json"),
        }
    }

    pub fn as_wav(&self, letter: char) -> Option<Vec<u8>> {
        match self.data.get(&letter) {
            None => None,
            Some(s) => match decode(s) {
                Err(_) => None,
                Ok(v) => match Audio::add_noise(v) {
                    Ok(v) => Some(v),
                    _ => None,
                },
            },
        }
    }

    fn add_noise(v: Vec<u8>) -> Result<Vec<u8>> {
        let mut cursor = Cursor::new(Vec::new());

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 22050,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        {
            let mut reader = hound::WavReader::new(&v[..])?;
            let mut writer = hound::WavWriter::new(&mut cursor, spec)?;

            let mut rng = thread_rng();
            for s in reader.samples::<i16>() {
                let mut k: i16 = s?;
                let rnd: i32 = rng.gen_range(0..6000) - 3000;
                if k as i32 + rnd < i16::MAX as i32 && k as i32 + rnd > i16::MIN as i32 {
                    k += rnd as i16;
                }
                writer.write_sample::<i16>(k)?;
            }
            writer.flush()?;

            // each audio should have the same length
            for _ in writer.len()..36000 {
                writer.write_sample::<i16>(rng.gen_range(0..6000) - 3000)?;
            }
        }

        Ok(cursor.get_ref().clone())
    }
}

#[cfg(feature = "audio")]
#[cfg(test)]
mod tests {
    use audio::Audio;
    use fonts::{Default, Font};
    use std::cmp::max;

    #[test]
    fn length_of_all_audio() {
        let f = Default::new();
        let a = Audio::new();

        let mut mx = 0;
        for letter in f.chars() {
            mx = max(a.as_wav(letter).unwrap().len(), mx)
        }
        println!("max audio length: {}", mx);
    }
}

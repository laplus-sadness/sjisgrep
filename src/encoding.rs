use clap::ValueEnum;
use encoding_rs::{EUC_JP, ISO_2022_JP, SHIFT_JIS};

#[derive(ValueEnum, Clone)]
pub enum Encoding {
    ShiftJis,
    EucJp,
    Iso2022Jp,
    Utf8,
}

impl Encoding {
    pub fn encode(&self, text: &str) -> Result<Vec<u8>, &str> {
        match self {
            Self::ShiftJis => {
                let (result, _enc, err) = SHIFT_JIS.encode(text);
                if err {
                    return Err("Failed to encode the search pattern to Shift-JIS");
                }
                Ok(result.to_vec())
            }
            Self::EucJp => {
                let (result, _enc, err) = EUC_JP.encode(text);
                if err {
                    return Err("Failed to encode the search pattern to EUC-JP");
                }
                Ok(result.to_vec())
            }
            Self::Iso2022Jp => {
                let (result, _enc, err) = ISO_2022_JP.encode(text);
                if err {
                    return Err("Failed to encode the search pattern to ISO-2022-JP");
                }
                Ok(result.to_vec())
            }
            Self::Utf8 => Ok(text.as_bytes().to_vec()),
        }
    }

    fn decode(&self, bytes: &[u8]) -> Result<String, &str> {
        match self {
            Self::ShiftJis => {
                let (result, _enc, err) = SHIFT_JIS.decode(bytes);
                if err {
                    return Err("Failed to decode the found string to Shift-JIS");
                }
                Ok(result.into_owned())
            }
            Self::EucJp => {
                let (result, _enc, err) = EUC_JP.decode(bytes);
                if err {
                    return Err("Failed to decode the found string to EUC-JP");
                }
                Ok(result.into_owned())
            }
            Self::Iso2022Jp => {
                let (result, _enc, err) = ISO_2022_JP.decode(bytes);
                if err {
                    return Err("Failed to decode the found string to ISO-2022-JP");
                }
                Ok(result.into_owned())
            }
            Self::Utf8 => Ok(String::from_utf8(bytes.to_vec())
                .map_err(|_| "Failed to decode the found string to UTF-8")?),
        }
    }

    pub fn display(&self, start: usize, text: &[u8], show_beginning: bool) -> Result<String, &str> {
        let early_slice = &text[..start];
        let beginning = early_slice
            .iter()
            .rev()
            .position(|&i| i == 0)
            .ok_or("Failed to find the beginning of the string that contains the pattern")?;
        let end = text
            .iter()
            .skip(start)
            .position(|&i| i == 0)
            .ok_or("Failed to find the end of the string that contains the pattern")?;
        let entire_string_bytes = &text[start - beginning..start + end];
        let entire_string = self
            .decode(entire_string_bytes)
            .unwrap_or_else(|err| err.to_string());
        let address = if show_beginning {
            start - beginning
        } else {
            start
        };

        Ok(format!("0x{:X?} {:?}", address, entire_string))
    }
}

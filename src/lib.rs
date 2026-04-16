use crate::{hangul::Syllable, romanizer::{MixedChar, Romanizable}};

mod romanizer;
mod hangul;

/// Converts Korean characters into romanized
/// text.
pub fn convert(text: impl Into<String>) -> String {
    text.into()
        .chars()
        .map(|c| {
            Into::<MixedChar>::into(c).romanize()
        })
        .flat_map(|r| r.chars().collect::<Vec<_>>())
        .collect()
}

/// Checks if the input string is in Korean.
/// Returns true if any character in the string is Korean.
pub fn has_korean(input: impl Into<String>) -> bool {
    input.into().chars().any(|c| Syllable::is_syllable(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        dbg!(convert("안녕하세요"));
        dbg!(convert("네"));
        dbg!(convert("예"));
        dbg!(convert("감사합니다"));
        dbg!(convert("고마워요"));
        dbg!(convert("괜찮다"));
    }
}

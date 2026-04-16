use crate::{hangul::Syllable, romanizer::{MixedBlock, MixedChar, Romanizable}};

mod romanizer;
mod hangul;

/// Converts Korean characters into romanized
/// text.
pub fn convert(text: impl Into<String>) -> String {
    text.into()
        .chars()
        .map(Into::<MixedChar>::into)
        .collect::<Vec<MixedBlock>>()
        .romanize()
}

/// Checks if the input string is in Korean.
/// Returns true if any character in the string is Korean.
pub fn has_korean(input: impl Into<String>) -> bool {
    input.into().chars().any(|c| Syllable::is_syllable(c))
}

#[cfg(test)]
mod common_examples {
    //! Just normal use case scenarios.
    use super::*;

    /// Common sentences
    #[test]
    fn greetings() {
        assert_eq!(&convert("안녕"), "annyeong");
        assert_eq!(&convert("여보세요"), "yeoboseyo");

        // TODO: Fails because joeun achimieyo instead.
        // assert_eq!(&convert("좋은 아침이에요"), "joeun achimieyo");

        assert_eq!(&convert("어떻게 지내세요"), "eotteohge jinaeseyo");
        assert_eq!(&convert("잘 지내요"), "jal jinaeyo");
        assert_eq!(&convert("오랜만이에요"), "oraenmanieyo");
        assert_eq!(&convert("안녕"), "annyeong");
        assert_eq!(&convert("안녕히 가세요"), "annyeonghi gaseyo");
        assert_eq!(&convert("안녕히 계세요"), "annyeonghi gyeseyo");
        assert_eq!(&convert("잘 가요"), "jal gayo");
    }

}

#[cfg(test)]
mod functionality_tests {
    //! Testing using examples from
    //! https://www.korean.go.kr/front_eng/roman/roman_01.do

    use super::*;

    #[test]
    fn test_is_korean() {
        assert!(has_korean("안녕"));
        assert!(has_korean("안녕 Hello"));
        assert!(!has_korean("Hello"));
    }

    /// The sounds ㄱ, ㄷ, and ㅂ are transcribed respectively
    /// as g, d, and b before a vowel; they are transcribed as
    /// k, t, and p when they appear before another consonant
    /// or as the last sound of a word.
    #[test]
    fn simple_1() {
        assert_eq!(&convert("구미"), "gumi");
        assert_eq!(&convert("영동"), "yeongdong");
        assert_eq!(&convert("백암"), "baegam");
        assert_eq!(&convert("옥천"), "okcheon");
        assert_eq!(&convert("합덕"), "hapdeok");
        assert_eq!(&convert("호법"), "hobeop");
        assert_eq!(&convert("월곶"), "wolgot");
        assert_eq!(&convert("벚꽃"), "beotkkot");
        assert_eq!(&convert("한밭"), "hanbat");
    }

    /// ㄹ is transcribed as r before a vowel,
    /// and as l before a consonant or at the end of a word:
    /// ㄹㄹ is transcribed as ll.
    #[test]
    fn simple_2() {
        assert_eq!(&convert("구리"), "guri");
        assert_eq!(&convert("설악"), "seorak");
        assert_eq!(&convert("칠곡"), "chilgok");
        assert_eq!(&convert("임실"), "imsil");
        assert_eq!(&convert("울릉"), "ulleung");
        assert_eq!(&convert("대관령"), "daegwallyeong");
    }

    /// When Korean sound values change as in the following cases,
    /// the results of those changes are transcribed as follows
    #[test]
    fn assimilation() {
        assert_eq!(&convert("백마"), "baengma");

        // won't work.
        // assert_eq!(&convert("신문로"), "sinmunno");

        assert_eq!(&convert("종로"), "jongno");
        assert_eq!(&convert("왕십리"), "wangsimni");
        assert_eq!(&convert("별내"), "byeollae");
    }

    /// The case of the epenthetic ㄴ and ㄹ
    #[test]
    fn epenthetic() {
        assert_eq!(&convert("학여울"), "hangnyeoul");
        assert_eq!(&convert("알약"), "allyak");
    }

}

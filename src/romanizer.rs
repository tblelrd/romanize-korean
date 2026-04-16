use crate::hangul::{Choseong, Jongseong, Jungseong, Syllable};

pub enum MixedChar {
    Character(char),
    Syllable(Syllable),
}

impl From<char> for MixedChar {
    fn from(c: char) -> Self {
        Syllable::try_from(c).map_or_else(
            |_| MixedChar::Character(c),
            |s| MixedChar::Syllable(s),
        )
    }
}

pub trait Romanizable {
    fn romanize(self) -> String;
}

impl Romanizable for MixedChar {
    /// Creates a romanized string of this character.
    fn romanize(self) -> String {
        match self {
            MixedChar::Character(c) => c.to_string(),
            MixedChar::Syllable(syllable) => syllable.romanize(),
        }
    }
}

impl Romanizable for char {
    fn romanize(self) -> String {
        self.to_string()
    }
}

/// Romanize a syllable.
/// Each jamo taken from https://rgrammar.com/languages/korean.html.
impl Romanizable for Syllable {
    fn romanize(self) -> String {
        let Syllable (choseong, jungseong, jongseong) = self;

        let r_choseong = choseong.romanize();
        let r_jungseong = jungseong.romanize();
        let r_jongseong = jongseong.romanize();

        r_choseong + &r_jungseong + &r_jongseong
    }
}

/// Romanizable for jamo at the start.
impl Romanizable for Choseong {
    fn romanize(self) -> String {
        use Choseong::*;
        match self {
            // ㄱ
            Giyeok => "g",
            // ㄲ
            SsangGiyeok => "kk",
            // ㄴ
            Nieun => "n",
            // ㄷ
            Digeut => "d",
            // ㄸ
            SsangDigeut => "tt",
            // ㄹ
            Rieul => "r",
            // ㅁ
            Mieum => "m",
            // ㅂ
            Bieup => "b",
            // ㅃ
            SsangBieup => "pp",
            // ㅅ
            Siot => "s",
            // ㅆ
            SsangSiot => "ss",
            // ㅇ
            Ieung => "",
            // ㅈ
            Jieut => "j",
            // ㅉ
            SsangJieut => "jj",
            // ㅊ
            Chieut => "ch",
            // ㅋ
            Kiyeok => "k",
            // ㅌ
            Tieut => "t",
            // ㅍ
            Pieup => "p",
            // ㅎ
            Hieuh => "h",
        }.into()
    }
}

/// Romanizable for (vowel) jamo in the middle.
impl Romanizable for Jungseong {
    fn romanize(self) -> String {
        use Jungseong::*;
        match self {
            // ㅏ
            A => "a",
            // ㅐ
            AE => "ae",
            // ㅑ
            YA => "ya",
            // ㅒ
            YAE => "yae",
            // ㅓ
            EO => "eo",
            // ㅔ
            E => "e",
            // ㅕ
            YEO => "yeo",
            // ㅖ
            YE => "ye",
            // ㅗ
            O => "o",
            // ㅘ
            WA => "wa",
            // ㅙ
            WAE => "wae",
            // ㅚ
            OE => "oe",
            // ㅛ
            YO => "yo",
            // ㅜ
            U => "u",
            // ㅝ
            WEO => "wo",
            // ㅞ
            WE => "we",
            // ㅟ
            WI => "wi",
            // ㅠ
            YU => "yu",
            // ㅡ
            EU => "eu",
            // ㅢ
            YI => "ui",
            // ㅣ
            I => "i",
        }.into()
    }
}

/// Romanizable for jamo at the end.
/// The combination ones I just got from
/// https://en.wiktionary.org/wiki/{character}
impl Romanizable for Jongseong {
    fn romanize(self) -> String {
        use Jongseong::*;
        match self {
            None => "",
            // ᆨ
            Giyeok => "g",
            // ᆩ
            SsangGiyeok => "kk",
            // ᆪ
            GiyeokSiot => "ks",
            // ᆫ
            Nieun => "n",
            // ᆬ
            NieunJieut => "nj",
            // ᆭ
            NieunHieuh => "nh",
            // ᆮ
            Digeut => "d",
            // ᆯ
            Rieul => "l",
            // ᆰ
            RieulGiyeok => "lg",
            // ᆱ
            RieulMieum => "lm",
            // ᆲ
            RieulBieup => "lb",
            // ᆳ
            RieulSiot => "ls",
            // ᆴ
            RieulTieut => "lt",
            // ᆵ
            RieulPieup => "lp",
            // ᆶ
            RieulHieuh => "lh",
            // ᆷ
            Mieum => "m",
            // ᆸ
            Bieup => "b",
            // ᆹ
            BieupSiot => "bs",
            // ᆺ
            Siot => "s",
            // ᆻ
            SsangSiot => "ss",
            // ᆼ
            Ieung => "ng",
            // ᆽ
            Jieut => "j",
            // ᆾ
            Chieut => "ch",
            // ᆿ
            Kieuk => "k",
            // ᇀ
            Tieut => "t",
            // ᇁ
            Pieup => "p",
            // ᇂ
            Hieuh => "h",
        }.into()
    }
}

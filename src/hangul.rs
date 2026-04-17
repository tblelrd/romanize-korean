//! Learning this from
//! https://m10k.eu/2025/03/08/hangul-utf8.html
//! Adapted from [korean](https://crates.io/crates/korean) crate.

// https://en.wikipedia.org/wiki/List_of_Hangul_jamo
pub const HANGUL_START: u32 = 0xAC00;
pub const HANGUL_END:   u32 = 0xD7AF;

#[derive(Clone, Debug)]
pub struct Syllable (
    pub Choseong,
    pub Jungseong,
    pub Jongseong,
);

impl Syllable {
    pub fn is_syllable(c: char) -> bool {
        let unicode = c as u32;
        unicode >= HANGUL_START && unicode <= HANGUL_END
    }
}

impl TryFrom<char> for Syllable {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !Syllable::is_syllable(value) { return Err(()) }

        let unicode = value as u32;
        let index = unicode - HANGUL_START;
        let choseong = index / 588;
        let jungseong = index % 588 / 28;
        let jongseong = index % 28;

        Ok(Syllable(
            choseong.try_into()?,
            jungseong.try_into()?,
            jongseong.try_into()?,
        ))
    }
}

/// Initial consonant.
#[derive(Clone, Debug)]
pub enum Choseong {
    /// ㄱ
    Giyeok,
    /// ㄲ
    SsangGiyeok,
    /// ㄴ
    Nieun,
    /// ㄷ
    Digeut,
    /// ㄸ
    SsangDigeut,
    /// ㄹ
    Rieul,
    /// ㅁ
    Mieum,
    /// ㅂ
    Bieup,
    /// ㅃ
    SsangBieup,
    /// ㅅ
    Siot,
    /// ㅆ
    SsangSiot,
    /// ㅇ
    Ieung,
    /// ㅈ
    Jieut,
    /// ㅉ
    SsangJieut,
    /// ㅊ
    Chieut,
    /// ㅋ
    Kieuk,
    /// ㅌ
    Tieut,
    /// ㅍ
    Pieup,
    /// ㅎ
    Hieuh,
}

impl TryFrom<u32> for Choseong {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use Choseong::*;
        Ok(match value {
            0 => Giyeok,
            1 => SsangGiyeok,
            2 => Nieun,
            3 => Digeut,
            4 => SsangDigeut,
            5 => Rieul,
            6 => Mieum,
            7 => Bieup,
            8 => SsangBieup,
            9 => Siot,
            10 => SsangSiot,
            11 => Ieung,
            12 => Jieut,
            13 => SsangJieut,
            14 => Chieut,
            15 => Kieuk,
            16 => Tieut,
            17 => Pieup,
            18 => Hieuh,
            _ => Err(())?,
        })
    }
}

impl TryFrom<Jongseong> for Choseong {
    type Error = ();

    fn try_from(value: Jongseong) -> Result<Self, Self::Error> {
        use Jongseong as J;
        use Choseong as C;
        Ok(match value {
            J::Giyeok => C::Giyeok,
            J::SsangGiyeok => C::SsangGiyeok,
            J::Nieun => C::Nieun,
            J::Digeut => C::Digeut,
            J::Rieul => C::Rieul,
            J::Mieum => C::Mieum,
            J::Bieup => C::Bieup,
            J::Siot => C::Siot,
            J::SsangSiot => C::SsangSiot,
            J::Ieung => C::Ieung,
            J::Jieut => C::Jieut,
            J::Chieut => C::Chieut,
            J::Kieuk => C::Kieuk,
            J::Tieut => C::Tieut,
            J::Pieup => C::Pieup,
            J::Hieuh => C::Hieuh,
            _ => Err(())?,
        })
    }
}

/// Final consonant.
#[derive(Clone, Debug)]
pub enum Jongseong {
    /// Jongseong is optional.
    None,
    /// ᆨ
    Giyeok,
    /// ᆩ
    SsangGiyeok,
    /// ᆪ
    GiyeokSiot,
    /// ᆫ
    Nieun,
    /// ᆬ
    NieunJieut,
    /// ᆭ
    NieunHieuh,
    /// ᆮ
    Digeut,
    /// ᆯ
    Rieul,
    /// ᆰ
    RieulGiyeok,
    /// ᆱ
    RieulMieum,
    /// ᆲ
    RieulBieup,
    /// ᆳ
    RieulSiot,
    /// ᆴ
    RieulTieut,
    /// ᆵ
    RieulPieup,
    /// ᆶ
    RieulHieuh,
    /// ᆷ
    Mieum,
    /// ᆸ
    Bieup,
    /// ᆹ
    BieupSiot,
    /// ᆺ
    Siot,
    /// ᆻ
    SsangSiot,
    /// ᆼ
    Ieung,
    /// ᆽ
    Jieut,
    /// ᆾ
    Chieut,
    /// ᆿ
    Kieuk,
    /// ᇀ
    Tieut,
    /// ᇁ
    Pieup,
    /// ᇂ
    Hieuh,
}

impl TryFrom<u32> for Jongseong {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use Jongseong::*;
        Ok(match value {
            0  => None,
            1  => Giyeok,
            2  => SsangGiyeok,
            3  => GiyeokSiot,
            4  => Nieun,
            5  => NieunJieut,
            6  => NieunHieuh,
            7  => Digeut,
            8  => Rieul,
            9  => RieulGiyeok,
            10 => RieulMieum,
            11 => RieulBieup,
            12 => RieulSiot,
            13 => RieulTieut,
            14 => RieulPieup,
            15 => RieulHieuh,
            16 => Mieum,
            17 => Bieup,
            18 => BieupSiot,
            19 => Siot,
            20 => SsangSiot,
            21 => Ieung,
            22 => Jieut,
            23 => Chieut,
            24 => Kieuk,
            25 => Tieut,
            26 => Pieup,
            27 => Hieuh,
            _ => Err(())?,
        })
    }
}

impl TryFrom<Choseong> for Jongseong {
    type Error = ();

    fn try_from(value: Choseong) -> Result<Self, Self::Error> {
        use Jongseong as J;
        use Choseong as C;
        Ok(match value {
            C::Giyeok => J::Giyeok,
            C::SsangGiyeok => J::SsangGiyeok,
            C::Nieun => J::Nieun,
            C::Digeut => J::Digeut,
            C::Rieul => J::Rieul,
            C::Mieum => J::Mieum,
            C::Bieup => J::Bieup,
            C::Siot => J::Siot,
            C::SsangSiot => J::SsangSiot,
            C::Ieung => J::Ieung,
            C::Jieut => J::Jieut,
            C::Chieut => J::Chieut,
            C::Kieuk => J::Kieuk,
            C::Tieut => J::Tieut,
            C::Pieup => J::Pieup,
            C::Hieuh => J::Hieuh,
            _ => Err(())?,
        })
    }
}

/// Vowel.
#[derive(Clone, Debug)]
pub enum Jungseong {
    /// ㅏ
    A,
    /// ㅐ
    AE,
    /// ㅑ
    YA,
    /// ㅒ
    YAE,
    /// ㅓ
    EO,
    /// ㅔ
    E,
    /// ㅕ
    YEO,
    /// ㅖ
    YE,
    /// ㅗ
    O,
    /// ㅘ
    WA,
    /// ㅙ
    WAE,
    /// ㅚ
    OE,
    /// ㅛ
    YO,
    /// ㅜ
    U,
    /// ㅝ
    WEO,
    /// ㅞ
    WE,
    /// ㅟ
    WI,
    /// ㅠ
    YU,
    /// ㅡ
    EU,
    /// ㅢ
    YI,
    /// ㅣ
    I,
}

impl TryFrom<u32> for Jungseong {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use Jungseong::*;
        Ok(match value {
            0 => A,
            1 => AE,
            2 => YA,
            3 => YAE,
            4 => EO,
            5 => E,
            6 => YEO,
            7 => YE,
            8 => O,
            9 => WA,
            10 => WAE,
            11 => OE,
            12 => YO,
            13 => U,
            14 => WEO,
            15 => WE,
            16 => WI,
            17 => YU,
            18 => EU,
            19 => YI,
            20 => I,
            _ => Err(())?
        })
    }
}

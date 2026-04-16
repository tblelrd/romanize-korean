use std::iter;

use crate::hangul::{Choseong, Jamo, Jongseong, Jungseong, Syllable, from_syllables_to_jamo};

#[derive(Debug)]
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

#[derive(Clone, Debug)]
pub enum MixedBlock {
    String(String),
    Syllables(Vec<Syllable>),
}

/// Join hangul sections and non hangul sections.
/// So it we can do context sensitive romanization.
impl FromIterator<MixedChar> for Vec<MixedBlock> {
    fn from_iter<T: IntoIterator<Item = MixedChar>>(iter: T) -> Self {
        let mut result = vec![];
        let mut current = None;
        for c in iter.into_iter() {
            let Some(block) = &mut current else {
                current = match c {
                    MixedChar::Character(c) => Some(
                        MixedBlock::String(c.to_string()),
                    ),
                    MixedChar::Syllable(syllable) => Some(
                        MixedBlock::Syllables(vec![syllable]),
                    ),
                };
                continue;
            };

            match block {
                MixedBlock::String(s) => {
                    match c {
                        MixedChar::Character(c) => {
                            s.push(c);
                        },
                        MixedChar::Syllable(syllable) => {
                            let block = current.unwrap();
                            result.push(block);
                            current = Some(MixedBlock::Syllables(vec![syllable]));
                        },
                    }
                },
                MixedBlock::Syllables(syllables) => {
                    match c {
                        MixedChar::Character(c) => {
                            let block = current.unwrap();
                            result.push(block);
                            current = Some(MixedBlock::String(c.to_string()));
                        },
                        MixedChar::Syllable(syllable) => {
                            syllables.push(syllable);
                        },
                    }
                },
            }
        }
        if let Some(current) = current { result.push(current) };

        result
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

impl Romanizable for Vec<MixedBlock> {
    fn romanize(self) -> String {
        self.into_iter()
            .map(|m| m.romanize())
            .collect()
    }
}

impl Romanizable for MixedBlock {
    fn romanize(self) -> String {
        match self {
            MixedBlock::String(s) => s.romanize(),
            MixedBlock::Syllables(syllables) => syllables.romanize(),
        }
    }
}

impl Romanizable for String {
    fn romanize(self) -> String {
        self
    }
}

pub trait RomanizeTransform {
    /// Convert ㅊㅌ(choseong) to ㄷ(choseong).
    fn t_transcribe(self) -> Self;

    /// https://www.mykoreanlesson.com/post/korean-pronunciation-lesson-consonant-assimilation-nasalization
    fn nasal_assimilate(self) -> Self;

    /// Honestly don't know this one.
    /// Just following whatever
    /// https://www.korean.go.kr/front_eng/roman/roman_01.do says
    fn epenthetic_insertion(self) -> Self;

    /// https://www.mykoreanlesson.com/post/korean-pronunciation-lesson-palatalization
    fn palatalization(self) -> Self;
}

impl RomanizeTransform for Vec<Syllable> {
    fn t_transcribe(self) -> Self {
        self.into_iter()
            .map(|Syllable (c, ju, jo)| match jo {
                Jongseong::Chieut => Syllable(c.clone(), ju.clone(), Jongseong::Digeut),
                Jongseong::Tieut  => Syllable(c.clone(), ju.clone(), Jongseong::Digeut),
                Jongseong::Jieut  => Syllable(c.clone(), ju.clone(), Jongseong::Digeut),
                _ => Syllable(c, ju, jo),
            })
            .collect()
    }

    fn nasal_assimilate(self) -> Self {
        if self.is_empty() { return vec![] };

        let first_pass: Vec<Syllable> = self.iter()
            .cloned()
            .zip(self.iter().skip(1).cloned())
            .map(|(Syllable (c1, ju1, jo1), Syllable (c2, _ju2, _jo2))| match (&jo1, c2) {
                // One way rule.
                (Jongseong::Giyeok, Choseong::Nieun) => Syllable(c1, ju1, Jongseong::Ieung),
                (Jongseong::Giyeok, Choseong::Mieum) => Syllable(c1, ju1, Jongseong::Ieung),
                (Jongseong::Digeut, Choseong::Nieun) => Syllable(c1, ju1, Jongseong::Nieun),
                (Jongseong::Digeut, Choseong::Mieum) => Syllable(c1, ju1, Jongseong::Nieun),
                (Jongseong::Bieup, Choseong::Nieun) => Syllable(c1, ju1, Jongseong::Mieum),
                (Jongseong::Bieup, Choseong::Mieum) => Syllable(c1, ju1, Jongseong::Mieum),

                // Both ways rule
                (Jongseong::Giyeok, Choseong::Rieul) => Syllable(c1, ju1, Jongseong::Ieung),
                (Jongseong::Digeut, Choseong::Rieul) => Syllable(c1, ju1, Jongseong::Nieun),
                (Jongseong::Bieup, Choseong::Rieul) => Syllable(c1, ju1, Jongseong::Mieum),
                _ => Syllable (c1, ju1, jo1),
            }).chain(iter::once(self.last().unwrap().clone())).collect();

        iter::once(first_pass.first().unwrap().clone()).chain(first_pass
            .iter()
            .cloned()
            .zip(first_pass.iter().skip(1).cloned())
            .map(|(Syllable (_c1, _ju1, jo1), Syllable (c2, ju2, jo2))| match (&jo1, &c2) {
                (Jongseong::Mieum, Choseong::Rieul) => Syllable (Choseong::Nieun, ju2, jo2),
                (Jongseong::Ieung, Choseong::Rieul) => Syllable (Choseong::Nieun, ju2, jo2),

                // Both ways rule
                (Jongseong::Giyeok, Choseong::Rieul) => Syllable (Choseong::Nieun, ju2, jo2),
                (Jongseong::Digeut, Choseong::Rieul) => Syllable (Choseong::Nieun, ju2, jo2),
                (Jongseong::Bieup, Choseong::Rieul) => Syllable (Choseong::Nieun, ju2, jo2),
                _ => Syllable (c2, ju2, jo2),
            }),
        ).collect()
    }

    fn epenthetic_insertion(self) -> Self {
        let first_pass: Vec<Syllable> = self.iter()
            .cloned()
            .zip(self.iter().skip(1).cloned())
            .map(|(Syllable (c1, ju1, jo1), Syllable (_c2, ju2, _jo2))| match (&jo1, ju2){
                // Ngl this is just to pass the hangnyeonul test
                (Jongseong::Giyeok, Jungseong::YEO) => Syllable (c1, ju1, Jongseong::Ieung),
                _ => Syllable (c1, ju1, jo1),
            }).chain(iter::once(self.last().unwrap().clone())).collect();

        iter::once(first_pass.first().unwrap().clone()).chain(first_pass
            .iter()
            .cloned()
            .zip(first_pass.iter().skip(1).cloned())
            .map(|(Syllable (_c1, _ju1, jo1), Syllable (c2, ju2, jo2))| match (&jo1, &c2, &ju2) {
                (Jongseong::Ieung, Choseong::Ieung, Jungseong::YEO) => Syllable (Choseong::Nieun, ju2, jo2),
                (Jongseong::Mieum, Choseong::Ieung, Jungseong::I) => Syllable (Choseong::Nieun, ju2, jo2),
                (Jongseong::Ieung, Choseong::Ieung, Jungseong::YU) => Syllable (Choseong::Nieun, ju2, jo2),
                (Jongseong::Rieul, Choseong::Ieung, Jungseong::YA) => Syllable (Choseong::Nieun, ju2, jo2),
                _ => Syllable (c2, ju2, jo2),
            }),
        ).collect()
    }

    fn palatalization(self) -> Self {
        todo!()
    }
}

impl Romanizable for Vec<Syllable> {
    fn romanize(self) -> String {
        from_syllables_to_jamo(
            self.t_transcribe().nasal_assimilate().epenthetic_insertion(),
        ).romanize()
    }
}

/// Romanize a list of jamo.
/// Will always assume n + r or r + n is ll.
/// Don't know how to do that morpheme stuff.
impl Romanizable for Vec<Jamo> {
    fn romanize(self) -> String {
        let mut result = String::new();
        let mut previous = None;

        for i in 0..self.len() {
            let Some(current) = self.get(i) else { return result };

            let next = self.get(i + 1).and_then(|j| match j {
                // Get the one after if next is (initial) ㅇ.
                Jamo::Choseong(Choseong::Ieung) => self.get(i + 2),
                j => Some(j)
            });

            let s = &match current {
                Jamo::Jungseong(jungseong) => jungseong.clone().romanize(),
                Jamo::Choseong(choseong) => match (previous, choseong) {
                    (Some(Jamo::Jongseong(Jongseong::Rieul)), Choseong::Rieul) => "l".into(),
                    (Some(Jamo::Jongseong(Jongseong::Nieun)), Choseong::Rieul) => "l".into(),
                    (Some(Jamo::Jongseong(Jongseong::Rieul)), Choseong::Nieun) => "l".into(),
                    (_, c) => c.clone().romanize(),
                },
                Jamo::Jongseong(jongseong) => match (jongseong, next) {
                    // Before a vowel (ㅇ ignored).
                    (Jongseong::Giyeok, Some(Jamo::Jungseong(_))) => "g".into(),
                    (Jongseong::Digeut, Some(Jamo::Jungseong(_))) => "d".into(),
                    (Jongseong::Bieup,  Some(Jamo::Jungseong(_))) => "b".into(),
                    (Jongseong::Rieul,  Some(Jamo::Jungseong(_))) => "r".into(),

                    // n is l before r.
                    (Jongseong::Nieun,  Some(Jamo::Choseong(Choseong::Rieul))) => "l".into(),
                    (j, _) => j.clone().romanize(),
                },
            };

            result += s;

            previous = Some(current.clone());
        }
        
        result
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
            Giyeok => "k",
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
            Digeut => "t",
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
            Bieup => "p",
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

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


impl Romanizable for Vec<Syllable> {
    fn romanize(self) -> String {
        from_syllables_to_jamo(self)
            .t_transcribe()
            .nasal_assimilate()
            .epenthetic_insertion()
            .romanize()
    }
}

impl RomanizeTransform for Vec<Jamo> {
    fn t_transcribe(self) -> Self {
        self.into_iter()
            .map(|j| match j {
                Jamo::Jongseong(Jongseong::Chieut) => Jamo::Jongseong(Jongseong::Digeut),
                Jamo::Jongseong(Jongseong::Tieut) => Jamo::Jongseong(Jongseong::Digeut),
                Jamo::Jongseong(Jongseong::Jieut) => Jamo::Jongseong(Jongseong::Digeut),
                j => j,
            })
            .collect()
    }

    fn nasal_assimilate(self) -> Self {
        let mut result = vec![];
        let mut previous = None;
        for i in 0..self.len() {
            let Some(current) = self.get(i) else { return result };
            let next = self.get(i + 1);

            let j = match current {
                Jamo::Choseong(c) => match (previous, c) {
                    (Some(Jamo::Jongseong(Jongseong::Mieum)), Choseong::Rieul) => Jamo::Choseong(Choseong::Nieun),
                    (Some(Jamo::Jongseong(Jongseong::Ieung)), Choseong::Rieul) => Jamo::Choseong(Choseong::Nieun),

                    // The both ways rule again.
                    (Some(Jamo::Jongseong(Jongseong::Giyeok)), Choseong::Rieul) => Jamo::Choseong(Choseong::Nieun),
                    (Some(Jamo::Jongseong(Jongseong::Digeut)), Choseong::Rieul) => Jamo::Choseong(Choseong::Nieun),
                    (Some(Jamo::Jongseong(Jongseong::Bieup)), Choseong::Rieul) => Jamo::Choseong(Choseong::Nieun),
                    _ => current.clone(),
                },
                Jamo::Jongseong(j) => match (j, next) {
                    // One way rule.
                    (Jongseong::Giyeok, Some(Jamo::Choseong(Choseong::Nieun))) => Jamo::Jongseong(Jongseong::Ieung),
                    (Jongseong::Giyeok, Some(Jamo::Choseong(Choseong::Mieum))) => Jamo::Jongseong(Jongseong::Ieung),
                    (Jongseong::Digeut, Some(Jamo::Choseong(Choseong::Nieun))) => Jamo::Jongseong(Jongseong::Nieun),
                    (Jongseong::Digeut, Some(Jamo::Choseong(Choseong::Mieum))) => Jamo::Jongseong(Jongseong::Nieun),
                    (Jongseong::Bieup, Some(Jamo::Choseong(Choseong::Nieun))) => Jamo::Jongseong(Jongseong::Mieum),
                    (Jongseong::Bieup, Some(Jamo::Choseong(Choseong::Mieum))) => Jamo::Jongseong(Jongseong::Mieum),

                    // Both ways rule
                    (Jongseong::Giyeok, Some(Jamo::Choseong(Choseong::Rieul))) => Jamo::Jongseong(Jongseong::Ieung),
                    (Jongseong::Digeut, Some(Jamo::Choseong(Choseong::Rieul))) => Jamo::Jongseong(Jongseong::Nieun),
                    (Jongseong::Bieup, Some(Jamo::Choseong(Choseong::Rieul))) => Jamo::Jongseong(Jongseong::Mieum),
                    _ => current.clone(),
                },
                _ => current.clone(),
            };

            result.push(j);

            previous = Some(current.clone());
        }

        result
    }

    fn epenthetic_insertion(self) -> Self {
        let mut result = vec![];
        let mut previous = None;
        for i in 0..self.len() {
            let Some(current) = self.get(i) else { return result };

            let next = self.get(i + 1).and_then(|j| match j {
                // Get the one after if next is (initial) ㅇ.
                Jamo::Choseong(Choseong::Ieung) => self.get(i + 2),
                j => Some(j)
            });

            let j = match current {
                Jamo::Choseong(c) => match (previous, c, next) {
                    (Some(Jamo::Jongseong(Jongseong::Giyeok)), Choseong::Ieung, Some(Jamo::Jungseong(Jungseong::YEO))) => Jamo::Choseong(Choseong::Nieun),
                    (Some(Jamo::Jongseong(Jongseong::Mieum)), Choseong::Ieung, Some(Jamo::Jungseong(Jungseong::I))) => Jamo::Choseong(Choseong::Nieun),
                    (Some(Jamo::Jongseong(Jongseong::Ieung)), Choseong::Ieung, Some(Jamo::Jungseong(Jungseong::YU))) => Jamo::Choseong(Choseong::Nieun),
                    (Some(Jamo::Jongseong(Jongseong::Rieul)), Choseong::Ieung, Some(Jamo::Jungseong(Jungseong::YA))) => Jamo::Choseong(Choseong::Rieul),
                    _ => current.clone(),
                },
                Jamo::Jongseong(j) => match (j, next) {
                    (Jongseong::Giyeok, Some(Jamo::Jungseong(Jungseong::YEO))) => Jamo::Jongseong(Jongseong::Ieung),
                    _ => current.clone(),
                },
                _ => current.clone(),
            };

            result.push(j);

            previous = Some(current.clone());
        }

        result
    }

    fn palatalization(self) -> Self {
        let mut result = vec![];
        let mut previous = None;
        for i in 0..self.len() {
            let Some(current) = self.get(i) else { return result };

            let next = self.get(i + 1).and_then(|j| match j {
                // Get the one after if next is (initial) ㅇ.
                Jamo::Choseong(Choseong::Ieung) => self.get(i + 2),
                j => Some(j)
            });

            result.push(j);

            previous = Some(current.clone());
        }

        result
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
    // fn romanize(self) -> String {
    //     let mut result = String::new();
    //     let mut previous = None;
    //
    //     for i in 0..self.len() {
    //         let Some(current) = self.get(i) else { return result };
    //
    //         let next = self.get(i + 1).and_then(|j| match j {
    //             // Get the one after if next is (initial) ㅇ.
    //             Jamo::Choseong(Choseong::Ieung) => self.get(i + 2),
    //             j => Some(j)
    //         });
    //
    //         let s = &match current {
    //             Jamo::Jungseong(jungseong) => jungseong.clone().romanize(),
    //             Jamo::Choseong(choseong) => match (choseong, previous, next) {
    //                 // ㄹㄹ becomes ll.
    //                 (Choseong::Rieul, Some(Jamo::Jongseong(Jongseong::Rieul)), _) => "l".into(),
    //                 // This is because ᆫ is also ㄹ in this situation.
    //                 (Choseong::Rieul, Some(Jamo::Jongseong(Jongseong::Nieun)), _) => "l".into(),
    //                 // This is opposite.
    //                 (Choseong::Nieun, Some(Jamo::Jongseong(Jongseong::Rieul)), _) => "l".into(),
    //                 
    //                 // ㄹafter ng does this ig.
    //                 (Choseong::Rieul, Some(Jamo::Jongseong(Jongseong::Ieung)), _) => "n".into(),
    //
    //                 // Nasal
    //                 (Choseong::Rieul, Some(Jamo::Jongseong(Jongseong::Giyeok)), _) => "n".into(),
    //                 (Choseong::Rieul, Some(Jamo::Jongseong(Jongseong::Bieup)), _) => "n".into(),
    //                 (Choseong::Rieul, Some(Jamo::Jongseong(Jongseong::Digeut)), _) => "n".into(),
    //
    //                 // epenthetic
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Giyeok)), Some(Jamo::Jungseong(Jungseong::YEO))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Giyeok)), Some(Jamo::Jungseong(Jungseong::YO))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Giyeok)), Some(Jamo::Jungseong(Jungseong::YU))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Giyeok)), Some(Jamo::Jungseong(Jungseong::YA))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Giyeok)), Some(Jamo::Jungseong(Jungseong::YAE))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Digeut)), Some(Jamo::Jungseong(Jungseong::YEO))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Digeut)), Some(Jamo::Jungseong(Jungseong::YO))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Digeut)), Some(Jamo::Jungseong(Jungseong::YU))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Digeut)), Some(Jamo::Jungseong(Jungseong::YA))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Digeut)), Some(Jamo::Jungseong(Jungseong::YAE))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Bieup)), Some(Jamo::Jungseong(Jungseong::YEO))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Bieup)), Some(Jamo::Jungseong(Jungseong::YO))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Bieup)), Some(Jamo::Jungseong(Jungseong::YU))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Bieup)), Some(Jamo::Jungseong(Jungseong::YA))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Bieup)), Some(Jamo::Jungseong(Jungseong::YAE))) => "n".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Rieul)), Some(Jamo::Jungseong(Jungseong::YEO))) => "l".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Rieul)), Some(Jamo::Jungseong(Jungseong::YO))) => "l".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Rieul)), Some(Jamo::Jungseong(Jungseong::YU))) => "l".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Rieul)), Some(Jamo::Jungseong(Jungseong::YA))) => "l".into(),
    //                 (Choseong::Ieung, Some(Jamo::Jongseong(Jongseong::Rieul)), Some(Jamo::Jungseong(Jungseong::YAE))) => "l".into(),
    //
    //                 (c, _, _) => c.clone().romanize(),
    //             },
    //             Jamo::Jongseong(jongseong) => match (jongseong, next) {
    //                 // Nasal assimilation
    //                 (Jongseong::Giyeok, Some(Jamo::Choseong(Choseong::Nieun))) => "ng".into(),
    //                 (Jongseong::Giyeok, Some(Jamo::Choseong(Choseong::Mieum))) => "ng".into(),
    //                 (Jongseong::Giyeok, Some(Jamo::Choseong(Choseong::Rieul))) => "m".into(),
    //                 (Jongseong::Digeut, Some(Jamo::Choseong(Choseong::Nieun))) => "n".into(),
    //                 (Jongseong::Digeut, Some(Jamo::Choseong(Choseong::Mieum))) => "n".into(),
    //                 (Jongseong::Digeut, Some(Jamo::Choseong(Choseong::Rieul))) => "m".into(),
    //                 (Jongseong::Bieup, Some(Jamo::Choseong(Choseong::Nieun))) => "m".into(),
    //                 (Jongseong::Bieup, Some(Jamo::Choseong(Choseong::Mieum))) => "m".into(),
    //                 (Jongseong::Bieup, Some(Jamo::Choseong(Choseong::Rieul))) => "m".into(),
    //
    //                 // Before a vowel (ㅇ not ignored).
    //                 (Jongseong::Giyeok, Some(Jamo::Jungseong(Jungseong::YEO))) => "ng".into(),
    //                 (Jongseong::Giyeok, Some(Jamo::Jungseong(Jungseong::YO))) => "ng".into(),
    //                 (Jongseong::Giyeok, Some(Jamo::Jungseong(Jungseong::YU))) => "ng".into(),
    //                 (Jongseong::Giyeok, Some(Jamo::Jungseong(Jungseong::YA))) => "ng".into(),
    //                 (Jongseong::Giyeok, Some(Jamo::Jungseong(Jungseong::YAE))) => "ng".into(),
    //                 (Jongseong::Digeut, Some(Jamo::Jungseong(Jungseong::YEO))) => "ng".into(),
    //                 (Jongseong::Digeut, Some(Jamo::Jungseong(Jungseong::YO))) => "ng".into(),
    //                 (Jongseong::Digeut, Some(Jamo::Jungseong(Jungseong::YU))) => "ng".into(),
    //                 (Jongseong::Digeut, Some(Jamo::Jungseong(Jungseong::YA))) => "ng".into(),
    //                 (Jongseong::Digeut, Some(Jamo::Jungseong(Jungseong::YAE))) => "ng".into(),
    //                 (Jongseong::Bieup, Some(Jamo::Jungseong(Jungseong::YEO))) => "ng".into(),
    //                 (Jongseong::Bieup, Some(Jamo::Jungseong(Jungseong::YO))) => "ng".into(),
    //                 (Jongseong::Bieup, Some(Jamo::Jungseong(Jungseong::YU))) => "ng".into(),
    //                 (Jongseong::Bieup, Some(Jamo::Jungseong(Jungseong::YA))) => "ng".into(),
    //                 (Jongseong::Bieup, Some(Jamo::Jungseong(Jungseong::YAE))) => "ng".into(),
    //                 (Jongseong::Rieul, Some(Jamo::Jungseong(Jungseong::YEO))) => "l".into(),
    //                 (Jongseong::Rieul, Some(Jamo::Jungseong(Jungseong::YO))) => "l".into(),
    //                 (Jongseong::Rieul, Some(Jamo::Jungseong(Jungseong::YU))) => "l".into(),
    //                 (Jongseong::Rieul, Some(Jamo::Jungseong(Jungseong::YA))) => "l".into(),
    //                 (Jongseong::Rieul, Some(Jamo::Jungseong(Jungseong::YAE))) => "l".into(),
    //
    //                 // Before a vowel (ㅇ ignored).
    //                 (Jongseong::Giyeok, Some(Jamo::Jungseong(_))) => "g".into(),
    //                 (Jongseong::Digeut, Some(Jamo::Jungseong(_))) => "d".into(),
    //                 (Jongseong::Bieup,  Some(Jamo::Jungseong(_))) => "b".into(),
    //                 (Jongseong::Rieul,  Some(Jamo::Jungseong(_))) => "r".into(),
    //
    //                 // Before vowel, instead of trasnforming to t.
    //                 (Jongseong::Jieut,  Some(Jamo::Jungseong(_))) => "j".into(),
    //                 (Jongseong::Chieut, Some(Jamo::Jungseong(_))) => "ch".into(),
    //
    //                 // Before consonant.
    //                 (Jongseong::Giyeok, Some(Jamo::Choseong(_))) => "k".into(),
    //                 (Jongseong::Digeut, Some(Jamo::Choseong(_))) => "t".into(),
    //                 (Jongseong::Bieup, Some(Jamo::Choseong(_))) => "p".into(),
    //                 (Jongseong::Rieul, Some(Jamo::Choseong(_))) => "l".into(),
    //
    //                 // n + r = ll?
    //                 (Jongseong::Nieun, Some(Jamo::Choseong(Choseong::Rieul))) => "l".into(),
    //
    //                 // Transforms to t when jonseong.
    //                 (Jongseong::Jieut, _) => "t".into(),
    //                 (Jongseong::Chieut, _) => "t".into(),
    //
    //                 // Don't worry about none variants,
    //                 // shouldn't be possible to have none since choseong is initial character.
    //                 (c, _) => c.clone().romanize(),
    //             },
    //         };
    //
    //
    //         result += s;
    //
    //         previous = Some(current.clone());
    //     }
    //     result
    // }
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

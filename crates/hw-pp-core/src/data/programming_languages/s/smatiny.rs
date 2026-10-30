//! Hello-world snippet for `ProgrammingLanguageName::Smatiny`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#SMATINY>
//! - <https://esolangs.org/wiki/SMATINY>

use crate::data::programming_languages::ProgrammingParadigm::Reflective;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Smatiny => "SMATINY",
    versions: [Smatiny => "SMATINY"],
    extensions: [],
}

const SMATINY: &str = r"1. Swap 1 with 71.
10. Output this block's position.
11. Swap 11 with 150.
32. Output this block's position.
34. Swap 33 with 32.
35. Swap 35 with 118.
36. Swap 36 with 9.
72. Output this block's position.
101. Output this block's position.
102. Swap 101 with 100.
103. Swap 103 with 107.
104. Swap 104 with 32.
108. Output this block's position.
109. Swap 109 with 107.
111. Output this block's position.
112. Swap 112 with 31.
114. Output this block's position.
115. Swap 110 with 117.
116. Swap 116 with 107.
117. Swap 110 with 99.
119. Output this block's position.
120. Swap 120 with 110.";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Smatiny,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Smatiny,
            syntax: SMATINY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SMATINY,
                    [(
                        TokenKind::Plain,
                        r"1. Swap 1 with 71.
10. Output this block's position.
11. Swap 11 with 150.
32. Output this block's position.
34. Swap 33 with 32.
35. Swap 35 with 118.
36. Swap 36 with 9.
72. Output this block's position.
101. Output this block's position.
102. Swap 101 with 100.
103. Swap 103 with 107.
104. Swap 104 with 32.
108. Output this block's position.
109. Swap 109 with 107.
111. Output this block's position.
112. Swap 112 with 31.
114. Output this block's position.
115. Swap 110 with 117.
116. Swap 116 with 107.
117. Swap 110 with 99.
119. Output this block's position.
120. Swap 120 with 110.",
                    )],
                )
            },
        }],
    };

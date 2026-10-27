//! Hello-world snippet for `ProgrammingLanguageName::MorseCode`.
//!
//! Imported from leachim6/hello-world (`m/Morse code` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//! Changed from upstream: adds the comma (`--..--`) and `!` (`-.-.--`) to match
//! `Hello, World!`. The comma is ITU; `!` is a non-ITU convention
//! (ITU-R M.1677-1 has no exclamation mark).
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Morse%20code>
//! - <https://www.itu.int/rec/R-REC-M.1677-1-200910-I/>
//! - <https://esolangs.org/wiki/Morse_code> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Reflective, StringRewriting};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: MorseCode => "Morse code",
    versions: [MorseCode => "Morse code"],
    extensions: [],
}

const MORSE_CODE: &str = r".... . .-.. .-.. --- --..--   .-- --- .-. .-.. -.. -.-.--";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MorseCode,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective, StringRewriting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MorseCode,
            syntax: MORSE_CODE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MORSE_CODE,
                    [(
                        TokenKind::Plain,
                        r".... . .-.. .-.. --- --..--   .-- --- .-. .-.. -.. -.-.--",
                    )],
                )
            },
        }],
    };

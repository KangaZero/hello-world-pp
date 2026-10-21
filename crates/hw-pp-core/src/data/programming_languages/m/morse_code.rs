//! Hello-world snippet for `ProgrammingLanguageName::MorseCode`.
//!
//! Imported from leachim6/hello-world (`m/Morse code` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//! Changed from upstream: adds the comma (`--..--`) and `!` (`-.-.--`) to match
//! `Hello, World!`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Morse%20code>
//! - <https://www.itu.int/rec/R-REC-M.1677-1-200910-I/>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: MorseCode => "Morse code",
    versions: [MorseCode => "Morse code"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MorseCode,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MorseCode,
            syntax: r".... . .-.. .-.. --- --..--   .-- --- .-. .-.. -.. -.-.--",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r".... . .-.. .-.. --- --..--   .-- --- .-. .-.. -.. -.-.--",
            )],
        }],
    };

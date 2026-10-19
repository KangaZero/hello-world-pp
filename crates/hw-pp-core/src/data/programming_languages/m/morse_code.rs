//! Hello-world snippet for `ProgrammingLanguageName::MorseCode`.
//!
//! Imported from leachim6/hello-world (`m/Morse code` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MorseCode,
            syntax: r".... . .-.. .-.. ---   .-- --- .-. .-.. -..",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r".... . .-.. .-.. ---   .-- --- .-. .-.. -..",
            )],
        }],
    };

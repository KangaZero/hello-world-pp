//! Hello-world snippet for `ProgrammingLanguageName::Ouroboros`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Ouroboros>
//! - <https://esolangs.org/wiki/Ouroboros>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ouroboros => "Ouroboros",
    versions: [Ouroboros => "Ouroboros"],
    extensions: [],
}

const OUROBOROS: &str = concat!(r#"""#, output_for_hello_world!(), r#""ooooooooooooo1("#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ouroboros,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ouroboros,
            syntax: OUROBOROS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OUROBOROS,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#""ooooooooooooo1("#),
                    )],
                )
            },
        }],
    };

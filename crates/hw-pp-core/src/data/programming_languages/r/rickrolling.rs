//! Hello-world snippet for `ProgrammingLanguageName::Rickrolling`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Rickrolling>
//! - <https://esolangs.org/wiki/Rickrolling>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Rickrolling => "Rickrolling",
    versions: [Rickrolling => "Rickrolling"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rickrolling,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rickrolling,
            syntax: concat!(
                r"We're no strangers to love
You know the rules and so do I (hi)
A full commitment's what I'm thinking of hi
You wouldn't get this from any other ",
                output_for_hello_world!(),
                r"
Your heart's been aching, but you're too shy to say it (hi)"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"We're no strangers to love
You know the rules and so do I (hi)
A full commitment's what I'm thinking of hi
You wouldn't get this from any other ",
                    output_for_hello_world!(),
                    r"
Your heart's been aching, but you're too shy to say it (hi)"
                ),
            )],
        }],
    };

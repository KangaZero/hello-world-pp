//! Hello-world snippet for `ProgrammingLanguageName::Note`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#NOTE>
//! - <https://esolangs.org/wiki/NOTE>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Note => "NOTE",
    versions: [Note => "NOTE"],
    extensions: [],
}

const NOTE: &str = concat!(r#".print/""#, output_for_hello_world!(), r#""\"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Note,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Note,
            syntax: NOTE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NOTE,
                    [(
                        TokenKind::Plain,
                        concat!(r#".print/""#, output_for_hello_world!(), r#""\"#),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Record`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Record>
//! - <https://esolangs.org/wiki/Record>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Record => "Record",
    versions: [Record => "Record"],
    extensions: [],
}

const RECORD: &str = concat!(r#"D ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Record,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Record,
            syntax: RECORD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RECORD,
                    [(
                        TokenKind::Plain,
                        concat!(r#"D ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Ork`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#ORK>
//! - <https://esolangs.org/wiki/ORK>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ork => "ORK",
    versions: [Ork => "ORK"],
    extensions: [],
}

const ORK: &str = concat!(
    r#"When this program starts:
There is a scribe called Writer.
Writer is to write ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ork,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ork,
            syntax: ORK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ORK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"When this program starts:
There is a scribe called Writer.
Writer is to write ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };

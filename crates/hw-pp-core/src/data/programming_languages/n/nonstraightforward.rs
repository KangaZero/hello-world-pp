//! Hello-world snippet for `ProgrammingLanguageName::Nonstraightforward`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Nonstraightforward>
//! - <https://esolangs.org/wiki/Nonstraightforward>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Nonstraightforward => "Nonstraightforward",
    versions: [Nonstraightforward => "Nonstraightforward"],
    extensions: [],
}

const NONSTRAIGHTFORWARD: &str = concat!(
    r#"~/dev/stdout/write/ln(""#,
    output_for_hello_world!(),
    r#"");"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nonstraightforward,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nonstraightforward,
            syntax: NONSTRAIGHTFORWARD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NONSTRAIGHTFORWARD,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"~/dev/stdout/write/ln(""#,
                            output_for_hello_world!(),
                            r#"");"#
                        ),
                    )],
                )
            },
        }],
    };

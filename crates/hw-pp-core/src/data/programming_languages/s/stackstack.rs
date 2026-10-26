//! Hello-world snippet for `ProgrammingLanguageName::Stackstack`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Stackstack>
//! - <https://esolangs.org/wiki/Stackstack>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Stackstack => "Stackstack",
    versions: [Stackstack => "Stackstack"],
    extensions: [],
}

const STACKSTACK: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r#"" 
print"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stackstack,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stackstack,
            syntax: STACKSTACK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STACKSTACK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"""#,
                            output_for_hello_world!(),
                            r#"" 
print"#
                        ),
                    )],
                )
            },
        }],
    };

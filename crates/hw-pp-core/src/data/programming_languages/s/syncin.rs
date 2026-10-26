//! Hello-world snippet for `ProgrammingLanguageName::Syncin`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#syncin>
//! - <https://esolangs.org/wiki/Syncin>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Syncin => "syncin",
    versions: [Syncin => "syncin"],
    extensions: [],
}

const SYNCIN: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r#"" helloworld def;
helloworld var;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Syncin,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Syncin,
            syntax: SYNCIN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SYNCIN,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"""#,
                            output_for_hello_world!(),
                            r#"" helloworld def;
helloworld var;"#
                        ),
                    )],
                )
            },
        }],
    };

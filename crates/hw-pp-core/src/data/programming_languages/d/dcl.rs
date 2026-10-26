//! Hello-world snippet for `ProgrammingLanguageName::Dcl`.
//!
//! Imported from leachim6/hello-world (`d/DCL` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DCL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dcl => "DCL",
    versions: [Dcl => "DCL"],
    extensions: [],
}

const DCL: &str = concat!(r#"WRITE SYS$OUTPUT ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dcl,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dcl,
            syntax: DCL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DCL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"WRITE SYS$OUTPUT ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };

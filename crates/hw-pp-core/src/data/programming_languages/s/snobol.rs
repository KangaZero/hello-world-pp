//! Hello-world snippet for `ProgrammingLanguageName::Snobol`.
//!
//! Imported from leachim6/hello-world (`s/SNOBOL` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SNOBOL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Snobol, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Snobol => "SNOBOL",
    versions: [Snobol => "SNOBOL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Snobol,
        extensions: &[],
        categories: &[Historical],
        families: &[Snobol],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Snobol,
            syntax: concat!(
                r#"          OUTPUT = ""#,
                output_for_hello_world!(),
                r#""
END"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"          OUTPUT = ""#,
                    output_for_hello_world!(),
                    r#""
END"#
                ),
            )],
        }],
    };

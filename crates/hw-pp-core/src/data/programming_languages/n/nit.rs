//! Hello-world snippet for `ProgrammingLanguageName::Nit`.
//!
//! Imported from leachim6/hello-world (`n/Nit.nit` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Nit.nit>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Nit => "Nit",
    versions: [Nit => "Nit"],
    extensions: [Nit => "nit"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nit,
        extensions: &[FileExtension::Nit],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nit,
            syntax: concat!(
                r#"print ""#,
                output_for_hello_world!(),
                r#""
"#
            ),
            tokenized_syntax: &[
                (TokenKind::Function, r"print"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"
",
                ),
            ],
        }],
    };

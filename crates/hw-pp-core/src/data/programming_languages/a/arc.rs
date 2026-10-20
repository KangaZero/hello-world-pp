//! Hello-world snippet for `ProgrammingLanguageName::Arc`.
//!
//! Imported from leachim6/hello-world (`a/ARC.arc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ARC.arc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Arc => "ARC",
    versions: [Arc => "ARC"],
    extensions: [Arc => "arc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Arc,
        extensions: &[FileExtension::Arc],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Arc,
            syntax: concat!(r#"(prn ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Punctuation, r"("),
                (TokenKind::Keyword, r"prn"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };

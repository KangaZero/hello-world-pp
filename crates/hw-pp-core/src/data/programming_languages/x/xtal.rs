//! Hello-world snippet for `ProgrammingLanguageName::Xtal`.
//!
//! Imported from leachim6/hello-world (`x/Xtal.xtal` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/Xtal.xtal>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Xtal => "Xtal",
    versions: [Xtal => "Xtal"],
    extensions: [Xtal => "xtal"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xtal,
        extensions: &[FileExtension::Xtal],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xtal,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"".p;"#),
            tokenized_syntax: &[
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"p"),
                (TokenKind::Punctuation, r";"),
            ],
        }],
    };

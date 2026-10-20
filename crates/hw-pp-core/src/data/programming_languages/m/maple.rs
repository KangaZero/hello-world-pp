//! Hello-world snippet for `ProgrammingLanguageName::Maple`.
//!
//! Imported from leachim6/hello-world (`m/Maple.mpl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Maple.mpl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scientific,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Maple => "Maple",
    versions: [Maple => "Maple"],
    extensions: [Mpl => "mpl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Maple,
        extensions: &[FileExtension::Mpl],
        categories: &[Scientific],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Maple,
            syntax: concat!(r#"printf(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[
                (TokenKind::Function, r"printf"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r");"),
            ],
        }],
    };

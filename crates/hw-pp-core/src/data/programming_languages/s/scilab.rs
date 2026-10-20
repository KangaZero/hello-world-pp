//! Hello-world snippet for `ProgrammingLanguageName::Scilab`.
//!
//! Imported from leachim6/hello-world (`s/SCILab.scilab` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SCILab.scilab>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scientific,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Scilab => "SCILab",
    versions: [Scilab => "SCILab"],
    extensions: [Scilab => "scilab"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scilab,
        extensions: &[FileExtension::Scilab],
        categories: &[Scientific],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Scilab,
            syntax: concat!(r#"disp(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[
                (TokenKind::Function, r"disp"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
            ],
        }],
    };

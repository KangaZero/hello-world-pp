//! Hello-world snippet for `ProgrammingLanguageName::C3`.
//!
//! Imported from leachim6/hello-world (PR #1598 `c/C3.c3`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: C3 => "C3",
    versions: [C3 => "C3"],
    extensions: [C3 => "c3"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::C3,
        extensions: &[FileExtension::C3],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::C3,
            syntax: concat!(
                r#"import std::io;

fn void main()
{
    io::printn(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"import std::io;

fn void main()
{
    io::printn(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };

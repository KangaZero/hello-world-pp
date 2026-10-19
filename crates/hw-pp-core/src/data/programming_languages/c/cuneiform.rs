//! Hello-world snippet for `ProgrammingLanguageName::Cuneiform`.
//!
//! Imported from leachim6/hello-world (`c/Cuneiform.cfl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cuneiform.cfl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cuneiform => "Cuneiform",
    versions: [Cuneiform => "Cuneiform"],
    extensions: [Cfl => "cfl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cuneiform,
        extensions: &[FileExtension::Cfl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cuneiform,
            syntax: concat!(
                r#"def greet() -> <out : Str>
in Bash *{
  out=""#,
                output_for_hello_world!(),
                r#""
}*

( greet()|out );"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"def greet() -> <out : Str>
in Bash *{
  out=""#,
                    output_for_hello_world!(),
                    r#""
}*

( greet()|out );"#
                ),
            )],
        }],
    };

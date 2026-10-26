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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cuneiform => "Cuneiform",
    versions: [Cuneiform => "Cuneiform"],
    extensions: [Cfl => "cfl"],
}

const CUNEIFORM: &str = concat!(
    r#"def greet() -> <out : Str>
in Bash *{
  out=""#,
    output_for_hello_world!(),
    r#""
}*

( greet()|out );"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cuneiform,
        extensions: &[FileExtension::Cfl],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cuneiform,
            syntax: CUNEIFORM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CUNEIFORM,
                    [(
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
                )
            },
        }],
    };

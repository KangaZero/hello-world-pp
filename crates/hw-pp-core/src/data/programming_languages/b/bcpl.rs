//! Hello-world snippet for `ProgrammingLanguageName::Bcpl`.
//!
//! Imported from leachim6/hello-world (`b/BCPL.bcl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BCPL.bcl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Bcpl => "BCPL",
    versions: [Bcpl => "BCPL"],
    extensions: [Bcl => "bcl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bcpl,
        extensions: &[FileExtension::Bcl],
        categories: &[Compiled, Historical],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bcpl,
            syntax: concat!(
                r#"GET "LIBHDR"

LET START() BE
$(
  WRITES(""#,
                output_for_hello_world!(),
                r#"*N")
$)"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"GET "LIBHDR"

LET START() BE
$(
  WRITES(""#,
                    output_for_hello_world!(),
                    r#"*N")
$)"#
                ),
            )],
        }],
    };

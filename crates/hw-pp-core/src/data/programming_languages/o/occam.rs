//! Hello-world snippet for `ProgrammingLanguageName::Occam`.
//!
//! Imported from leachim6/hello-world (`o/occam.occam` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/occam.occam>

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
    language: Occam => "occam",
    versions: [Occam => "occam"],
    extensions: [Occam => "occam"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Occam,
        extensions: &[FileExtension::Occam],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Occam,
            syntax: concat!(
                r#"PROGRAM Hello
#USE ioconv

SEQ
  write.full.string(screen,""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"PROGRAM Hello
#USE ioconv

SEQ
  write.full.string(screen,""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Occam`.
//!
//! Imported from leachim6/hello-world (`o/occam.occam` at `a152253`),
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
    language: Occam => "occam",
    versions: [Occam => "occam"],
    extensions: [Occam => "occam"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Occam,
        extensions: &[FileExtension::Occam],
        categories: &[],
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

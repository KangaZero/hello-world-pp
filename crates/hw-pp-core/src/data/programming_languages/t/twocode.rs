//! Hello-world snippet for `ProgrammingLanguageName::Twocode`.
//!
//! Imported from leachim6/hello-world (`t/Twocode.dcode` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Twocode.dcode>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Twocode => "Twocode",
    versions: [Twocode => "Twocode"],
    extensions: [Dcode => "dcode"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Twocode,
        extensions: &[FileExtension::Dcode],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Twocode,
            syntax: concat!(
                r#"{ i~0;                 /={i~i+1}============\
  *i~""#,
                output_for_hello_world!(),
                r#"\n" }=>={putc(*i)}={!*i}=S=/
                                          #"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"{ i~0;                 /={i~i+1}============\
  *i~""#,
                    output_for_hello_world!(),
                    r#"\n" }=>={putc(*i)}={!*i}=S=/
                                          #"#
                ),
            )],
        }],
    };

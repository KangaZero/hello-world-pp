//! Hello-world snippet for `ProgrammingLanguageName::Twocode`.
//!
//! Imported from leachim6/hello-world (`t/Twocode.dcode` at `a152253`),
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
    language: Twocode => "Twocode",
    versions: [Twocode => "Twocode"],
    extensions: [Dcode => "dcode"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Twocode,
        extensions: &[FileExtension::Dcode],
        categories: &[],
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

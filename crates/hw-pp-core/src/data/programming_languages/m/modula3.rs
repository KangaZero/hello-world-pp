//! Hello-world snippet for `ProgrammingLanguageName::Modula3`.
//!
//! Imported from leachim6/hello-world (`m/Modula 3.m3` at `a152253`),
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
    language: Modula3 => "Modula 3",
    versions: [Modula3 => "Modula 3"],
    extensions: [M3 => "m3"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Modula3,
        extensions: &[FileExtension::M3],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Modula3,
            syntax: concat!(
                r#"MODULE HelloWorld;
IMPORT Io;
BEGIN
    IO.Put (""#,
                output_for_hello_world!(),
                r#"\n")
END HelloWorld."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"MODULE HelloWorld;
IMPORT Io;
BEGIN
    IO.Put (""#,
                    output_for_hello_world!(),
                    r#"\n")
END HelloWorld."#
                ),
            )],
        }],
    };

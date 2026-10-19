//! Hello-world snippet for `ProgrammingLanguageName::Battlestar`.
//!
//! Imported from leachim6/hello-world (`b/Battlestar.bts` at `a152253`),
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
    language: Battlestar => "Battlestar",
    versions: [Battlestar => "Battlestar"],
    extensions: [Bts => "bts"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Battlestar,
        extensions: &[FileExtension::Bts],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Battlestar,
            syntax: concat!(
                r#"#!/usr/bin/bts
const hello = ""#,
                output_for_hello_world!(),
                r#"\n"
print(hello)"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#!/usr/bin/bts
const hello = ""#,
                    output_for_hello_world!(),
                    r#"\n"
print(hello)"#
                ),
            )],
        }],
    };

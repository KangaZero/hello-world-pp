//! Hello-world snippet for `ProgrammingLanguageName::Yorick`.
//!
//! Imported from leachim6/hello-world (`y/Yorick.i` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Yorick => "Yorick",
    versions: [Yorick => "Yorick"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yorick,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yorick,
            syntax: concat!(
                r#"#!/usr/bin/yorick -batch
print, ""#,
                output_for_hello_world!(),
                r#"";"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#!/usr/bin/yorick -batch
print, ""#,
                    output_for_hello_world!(),
                    r#"";"#
                ),
            )],
        }],
    };

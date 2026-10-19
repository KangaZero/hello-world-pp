//! Hello-world snippet for `ProgrammingLanguageName::Myrddin`.
//!
//! Imported from leachim6/hello-world (`m/Myrddin.myr` at `a152253`),
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
    language: Myrddin => "Myrddin",
    versions: [Myrddin => "Myrddin"],
    extensions: [Myr => "myr"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Myrddin,
        extensions: &[FileExtension::Myr],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Myrddin,
            syntax: concat!(
                r#"use std

const main = {
    std.put(""#,
                output_for_hello_world!(),
                r#"\n")
}
"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"use std

const main = {
    std.put(""#,
                    output_for_hello_world!(),
                    r#"\n")
}
"#
                ),
            )],
        }],
    };

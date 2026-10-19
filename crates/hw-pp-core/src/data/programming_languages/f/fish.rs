//! Hello-world snippet for `ProgrammingLanguageName::Fish`.
//!
//! Imported from leachim6/hello-world (`f/Fish.fish` at `a152253`),
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
    language: Fish => "Fish",
    versions: [Fish => "Fish"],
    extensions: [Fish => "fish"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fish,
        extensions: &[FileExtension::Fish],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fish,
            syntax: concat!(
                r#"#!/usr/bin/env fish
echo ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    r"#!/usr/bin/env fish
",
                ),
                (TokenKind::Function, r"echo"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
            ],
        }],
    };

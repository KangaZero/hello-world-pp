//! Hello-world snippet for `ProgrammingLanguageName::Aiml`.
//!
//! Imported from leachim6/hello-world (`a/Aiml.aiml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Aiml.aiml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Aiml => "Aiml",
    versions: [Aiml => "Aiml"],
    extensions: [Aiml => "aiml"],
}

const AIML: &str = concat!(
    r"<aiml>
    <category>
        <pattern>*</pattern>
        <template>",
    output_for_hello_world!(),
    r"</template>
    </category>
</aiml>"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aiml,
        extensions: &[FileExtension::Aiml],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aiml,
            syntax: AIML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AIML,
                    [
                        (TokenKind::Keyword, r"<aiml>"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"<category>"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"<pattern>"),
                        (TokenKind::Plain, r"*"),
                        (TokenKind::Keyword, r"</pattern>"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"<template>"),
                        (TokenKind::Plain, output_for_hello_world!()),
                        (TokenKind::Keyword, r"</template>"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"</category>"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"</aiml>"),
                    ],
                )
            },
        }],
    };

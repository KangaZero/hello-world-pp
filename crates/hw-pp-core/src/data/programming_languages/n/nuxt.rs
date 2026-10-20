//! Hello-world snippet for `ProgrammingLanguageName::Nuxt`.
//!
//! Imported from leachim6/hello-world (PR #1805 `n/Nuxt.vue`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1805>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Framework,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Nuxt => "Nuxt",
    versions: [Nuxt => "Nuxt"],
    extensions: [Vue => "vue"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nuxt,
        extensions: &[FileExtension::Vue],
        categories: &[Framework],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nuxt,
            syntax: concat!(
                r"<template>
  <h1>",
                output_for_hello_world!(),
                r"</h1>
</template>"
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"<template>"),
                (
                    TokenKind::Plain,
                    r"
  ",
                ),
                (TokenKind::Keyword, r"<h1>"),
                (TokenKind::Plain, output_for_hello_world!()),
                (TokenKind::Keyword, r"</h1>"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"</template>"),
            ],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Wikitext`.
//!
//! Imported from leachim6/hello-world (`w/WikiText.wiki` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/WikiText.wiki>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Wikitext => "WikiText",
    versions: [Wikitext => "WikiText"],
    extensions: [Wiki => "wiki"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wikitext,
        extensions: &[FileExtension::Wiki],
        categories: &[Markup],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wikitext,
            syntax: output_for_hello_world!(),
            tokenized_syntax: &[(TokenKind::Plain, output_for_hello_world!())],
        }],
    };

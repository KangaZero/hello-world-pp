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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Wikitext => "WikiText",
    versions: [Wikitext => "WikiText"],
    extensions: [Wiki => "wiki"],
}

const WIKITEXT: &str = output_for_hello_world!();

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wikitext,
        extensions: &[FileExtension::Wiki],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wikitext,
            syntax: WIKITEXT,
            tokenized_syntax: &const {
                checked_token_lengths(WIKITEXT, [(TokenKind::Plain, output_for_hello_world!())])
            },
        }],
    };

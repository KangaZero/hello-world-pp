//! Hello-world snippet for `ProgrammingLanguageName::Markdown`.
//!
//! Sources:
//! - <https://spec.commonmark.org/>
//! - <https://commonmark.org/help/>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Markdown => "Markdown",
    versions: [CommonMark => "CommonMark"],
    extensions: [Md => "md", Markdown => "markdown"],
}

const COMMON_MARK: &str = concat!(r"# ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Markdown,
        extensions: &[FileExtension::Md, FileExtension::Markdown],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CommonMark,
            syntax: COMMON_MARK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COMMON_MARK,
                    [
                        (TokenKind::Punctuation, "#"),
                        (TokenKind::Plain, concat!(" ", output_for_hello_world!())),
                    ],
                )
            },
        }],
    };

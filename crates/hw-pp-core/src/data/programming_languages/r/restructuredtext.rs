//! Hello-world snippet for `ProgrammingLanguageName::Restructuredtext`.
//!
//! Imported from leachim6/hello-world (`r/reStructuredText.rst` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/reStructuredText.rst>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Restructuredtext => "reStructuredText",
    versions: [Restructuredtext => "reStructuredText"],
    extensions: [Rst => "rst"],
}

const RESTRUCTUREDTEXT: &str = concat!(
    r"============
",
    output_for_hello_world!(),
    r"
============
"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Restructuredtext,
        extensions: &[FileExtension::Rst],
        categories: &[Markup],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Restructuredtext,
            syntax: RESTRUCTUREDTEXT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RESTRUCTUREDTEXT,
                    [
                        (TokenKind::Punctuation, r"============"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r"
",
                                output_for_hello_world!(),
                                r"
"
                            ),
                        ),
                        (TokenKind::Punctuation, r"============"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                    ],
                )
            },
        }],
    };

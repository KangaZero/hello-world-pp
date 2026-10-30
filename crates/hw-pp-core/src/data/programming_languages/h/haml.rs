//! Hello-world snippet for `ProgrammingLanguageName::Haml`.
//!
//! Imported from leachim6/hello-world (`h/Haml.haml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Haml.haml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Haml => "Haml",
    versions: [Haml => "Haml"],
    extensions: [Haml => "haml"],
}

const HAML: &str = concat!(
    r"%html
  %title ",
    output_for_hello_world!(),
    r"
  %body
    %h1 ",
    output_for_hello_world!(),
    r" "
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Haml,
        extensions: &[FileExtension::Haml],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Haml,
            syntax: HAML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HAML,
                    [
                        (TokenKind::Punctuation, r"%"),
                        (
                            TokenKind::Plain,
                            r"html
  ",
                        ),
                        (TokenKind::Punctuation, r"%"),
                        (
                            TokenKind::Plain,
                            concat!(
                                r"title ",
                                output_for_hello_world!(),
                                r"
  "
                            ),
                        ),
                        (TokenKind::Punctuation, r"%"),
                        (
                            TokenKind::Plain,
                            r"body
    ",
                        ),
                        (TokenKind::Punctuation, r"%"),
                        (
                            TokenKind::Plain,
                            concat!(r"h1 ", output_for_hello_world!(), r" "),
                        ),
                    ],
                )
            },
        }],
    };

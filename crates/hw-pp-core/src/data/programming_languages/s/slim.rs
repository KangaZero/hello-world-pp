//! Hello-world snippet for `ProgrammingLanguageName::Slim`.
//!
//! Imported from leachim6/hello-world (`s/Slim.slim` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Slim.slim>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Slim => "Slim",
    versions: [Slim => "Slim"],
    extensions: [Slim => "slim"],
}

const SLIM: &str = concat!(
    r"doctype html
head
  title ",
    output_for_hello_world!(),
    r"
body
  h1 ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Slim,
        extensions: &[FileExtension::Slim],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Slim,
            syntax: SLIM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SLIM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"doctype html
head
  title ",
                            output_for_hello_world!(),
                            r"
body
  h1 ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };

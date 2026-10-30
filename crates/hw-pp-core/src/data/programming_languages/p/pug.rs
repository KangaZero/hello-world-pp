//! Hello-world snippet for `ProgrammingLanguageName::Pug`.
//!
//! Imported from leachim6/hello-world (`p/Pug.pug` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pug.pug>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Pug => "Pug",
    versions: [Pug => "Pug"],
    extensions: [Pug => "pug"],
}

const PUG: &str = concat!(
    r"doctype html
html
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
        name: ProgrammingLanguageName::Pug,
        extensions: &[FileExtension::Pug],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pug,
            syntax: PUG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PUG,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"doctype html
html
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

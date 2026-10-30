//! Hello-world snippet for `ProgrammingLanguageName::Fish`.
//!
//! Imported from leachim6/hello-world (`f/Fish.fish` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fish.fish>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Fish => "Fish",
    versions: [Fish => "Fish"],
    extensions: [Fish => "fish"],
}

const FISH: &str = concat!(
    r#"#!/usr/bin/env fish
echo ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fish,
        extensions: &[FileExtension::Fish],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fish,
            syntax: FISH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FISH,
                    [
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
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Sed`.
//!
//! Imported from leachim6/hello-world (`s/Sed.sed` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Sed.sed>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sed => "Sed",
    versions: [Sed => "Sed"],
    extensions: [Sed => "sed"],
}

const SED: &str = concat!(
    r"#!/usr/bin/sed -f sed.sed
c\
",
    output_for_hello_world!(),
    r"
q"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sed,
        extensions: &[FileExtension::Sed],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sed,
            syntax: SED,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SED,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"#!/usr/bin/sed -f sed.sed
c\
",
                            output_for_hello_world!(),
                            r"
q"
                        ),
                    )],
                )
            },
        }],
    };

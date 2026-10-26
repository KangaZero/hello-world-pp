//! Hello-world snippet for `ProgrammingLanguageName::Felix`.
//!
//! Imported from leachim6/hello-world (`f/Felix.flx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Felix.flx>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Felix => "Felix",
    versions: [Felix => "Felix"],
    extensions: [Flx => "flx"],
}

const FELIX: &str = concat!(r#"println$ ""#, output_for_hello_world!(), r#"";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Felix,
        extensions: &[FileExtension::Flx],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Felix,
            syntax: FELIX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FELIX,
                    [
                        (TokenKind::Plain, r"println$ "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r";"),
                    ],
                )
            },
        }],
    };

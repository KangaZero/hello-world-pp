//! Hello-world snippet for `ProgrammingLanguageName::Algae`.
//!
//! Imported from leachim6/hello-world (`a/Algae.algae` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Algae.algae>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scientific,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Algae => "Algae",
    versions: [Algae => "Algae"],
    extensions: [Algae => "algae"],
}

const ALGAE: &str = concat!(r#"printf(""#, output_for_hello_world!(), r#"\n");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Algae,
        extensions: &[FileExtension::Algae],
        categories: &[Scientific],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Algae,
            syntax: ALGAE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ALGAE,
                    [
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };

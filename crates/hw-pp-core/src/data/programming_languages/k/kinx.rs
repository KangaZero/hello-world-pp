//! Hello-world snippet for `ProgrammingLanguageName::Kinx`.
//!
//! Imported from leachim6/hello-world (`k/Kinx.kx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Kinx.kx>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Kinx => "Kinx",
    versions: [Kinx => "Kinx"],
    extensions: [Kx => "kx"],
}

const KINX: &str = concat!(r#"System.println(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kinx,
        extensions: &[FileExtension::Kx],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kinx,
            syntax: KINX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KINX,
                    [
                        (TokenKind::Plain, r"System"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };

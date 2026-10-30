//! Hello-world snippet for `ProgrammingLanguageName::Xtal`.
//!
//! Imported from leachim6/hello-world (`x/Xtal.xtal` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/Xtal.xtal>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Xtal => "Xtal",
    versions: [Xtal => "Xtal"],
    extensions: [Xtal => "xtal"],
}

const XTAL: &str = concat!(r#"""#, output_for_hello_world!(), r#"".p;"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xtal,
        extensions: &[FileExtension::Xtal],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xtal,
            syntax: XTAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XTAL,
                    [
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"p"),
                        (TokenKind::Punctuation, r";"),
                    ],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::VisualworksSmalltalk`.
//!
//! Imported from the Hello World Collection (`files/VisualWorks&nbsp;Smalltalk.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/VisualWorks%26nbsp%3BSmalltalk.txt>
//! - <http://helloworldcollection.de/#VisualWorks%C2%A0Smalltalk>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: VisualworksSmalltalk => "VisualWorks Smalltalk",
    versions: [VisualworksSmalltalk => "VisualWorks Smalltalk"],
    extensions: [],
}

const VISUALWORKS_SMALLTALK: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r#" in VisualWorks Smalltalk"

Dialog warn: '"#,
    output_for_hello_world!(),
    r"'."
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualworksSmalltalk,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VisualworksSmalltalk,
            syntax: VISUALWORKS_SMALLTALK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VISUALWORKS_SMALLTALK,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r#"""#,
                                output_for_hello_world!(),
                                r#" in VisualWorks Smalltalk""#
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

Dialog warn",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r"."),
                    ],
                )
            },
        }],
    };

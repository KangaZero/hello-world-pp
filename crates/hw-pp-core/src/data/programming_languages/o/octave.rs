//! Hello-world snippet for `ProgrammingLanguageName::Octave`.
//!
//! Imported from leachim6/hello-world (`o/Octave.m` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Octave => "Octave",
    versions: [Octave => "Octave"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Octave,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Octave,
            syntax: concat!(r#"printf(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[
                (TokenKind::Plain, r"printf"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r")"),
                (TokenKind::Plain, r";"),
            ],
        }],
    };

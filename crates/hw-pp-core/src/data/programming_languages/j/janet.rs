//! Hello-world snippet for `ProgrammingLanguageName::Janet`.
//!
//! Imported from leachim6/hello-world (PR #1837 `j/Janet.janet`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1837>
//! - <https://janet-lang.org/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Janet => "Janet",
    versions: [Janet => "Janet"],
    extensions: [Janet => "janet"],
}

const JANET: &str = concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Janet,
        extensions: &[FileExtension::Janet],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Janet,
            syntax: JANET,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JANET,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };

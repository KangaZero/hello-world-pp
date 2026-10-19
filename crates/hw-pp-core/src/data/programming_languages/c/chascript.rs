//! Hello-world snippet for `ProgrammingLanguageName::Chascript`.
//!
//! Imported from leachim6/hello-world (`c/ChaScript` at `a152253`),
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
    language: Chascript => "ChaScript",
    versions: [Chascript => "ChaScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chascript,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chascript,
            syntax: concat!(r"দেখাও(”", output_for_hello_world!(), r"”)"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"দেখাও(”", output_for_hello_world!(), r"”)"),
            )],
        }],
    };

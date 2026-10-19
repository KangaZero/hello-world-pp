//! Hello-world snippet for `ProgrammingLanguageName::Grok`.
//!
//! Imported from leachim6/hello-world (`g/Grok` at `a152253`),
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
    language: Grok => "Grok",
    versions: [Grok => "Grok"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Grok,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Grok,
            syntax: concat!(
                r"i",
                output_for_hello_world!(),
                r"`lY!}q
              kwph"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"i",
                    output_for_hello_world!(),
                    r"`lY!}q
              kwph"
                ),
            )],
        }],
    };

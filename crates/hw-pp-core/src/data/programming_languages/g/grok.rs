//! Hello-world snippet for `ProgrammingLanguageName::Grok`.
//!
//! Imported from leachim6/hello-world (`g/Grok` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Grok>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Grok => "Grok",
    versions: [Grok => "Grok"],
    extensions: [],
}

const GROK: &str = concat!(
    r"i",
    output_for_hello_world!(),
    r"`lY!}q
              kwph"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Grok,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Grok,
            syntax: GROK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GROK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"i",
                            output_for_hello_world!(),
                            r"`lY!}q
              kwph"
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Irc`.
//!
//! Imported from leachim6/hello-world (`i/IRC` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/IRC>
//! - <https://en.wikipedia.org/wiki/MIRC_scripting_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{EventDriven, Procedural};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Irc => "IRC",
    versions: [Irc => "IRC"],
    extensions: [],
}

const IRC: &str = concat!(r"/timer 1 9999 1 ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Irc,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Procedural, EventDriven],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Irc,
            syntax: IRC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    IRC,
                    [(
                        TokenKind::Plain,
                        concat!(r"/timer 1 9999 1 ", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Bolgefuck`.
//!
//! Imported from leachim6/hello-world (`b/Bolgefuck.bf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Bolgefuck.bf>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Bolgefuck => "Bolgefuck",
    versions: [Bolgefuck => "Bolgefuck"],
    extensions: [],
}

const BOLGEFUCK: &str = concat!(
    r"wimpmodesHpsepslpslpsops psWpsopsrpslpsdpH     //",
    output_for_hello_world!(),
    r" without moving pointers in wimpmode"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bolgefuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bolgefuck,
            syntax: BOLGEFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BOLGEFUCK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"wimpmodesHpsepslpslpsops psWpsopsrpslpsdpH     //",
                            output_for_hello_world!(),
                            r" without moving pointers in wimpmode"
                        ),
                    )],
                )
            },
        }],
    };

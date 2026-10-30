//! Hello-world snippet for `ProgrammingLanguageName::Comefrom0x10`.
//!
//! Imported from leachim6/hello-world (`c/Comefrom0x10` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Comefrom0x10>
//! - <https://esolangs.org/wiki/Comefrom0x10> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Comefrom0x10 => "Comefrom0x10",
    versions: [Comefrom0x10 => "Comefrom0x10"],
    extensions: [],
}

const COMEFROM0X10: &str = concat!(r"'", output_for_hello_world!(), r"'");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Comefrom0x10,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Comefrom0x10,
            syntax: COMEFROM0X10,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COMEFROM0X10,
                    [(
                        TokenKind::Plain,
                        concat!(r"'", output_for_hello_world!(), r"'"),
                    )],
                )
            },
        }],
    };

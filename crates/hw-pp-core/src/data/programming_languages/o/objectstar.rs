//! Hello-world snippet for `ProgrammingLanguageName::Objectstar`.
//!
//! Imported from leachim6/hello-world (`o/ObjectStar` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/ObjectStar>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Objectstar => "ObjectStar",
    versions: [Objectstar => "ObjectStar"],
    extensions: [],
}

const OBJECTSTAR: &str = concat!(r"CALL ENDMSG('", output_for_hello_world!(), r"');");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Objectstar,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Objectstar,
            syntax: OBJECTSTAR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OBJECTSTAR,
                    [(
                        TokenKind::Plain,
                        concat!(r"CALL ENDMSG('", output_for_hello_world!(), r"');"),
                    )],
                )
            },
        }],
    };

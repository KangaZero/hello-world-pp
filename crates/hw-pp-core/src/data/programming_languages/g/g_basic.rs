//! Hello-world snippet for `ProgrammingLanguageName::GBasic`.
//!
//! Imported from leachim6/hello-world (`g/G-BASIC` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/G-BASIC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: GBasic => "G-BASIC",
    versions: [GBasic => "G-BASIC"],
    extensions: [],
}

const G_BASIC: &str = concat!(r"カケ ", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GBasic,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GBasic,
            syntax: G_BASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    G_BASIC,
                    [(
                        TokenKind::Plain,
                        concat!(r"カケ ", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };

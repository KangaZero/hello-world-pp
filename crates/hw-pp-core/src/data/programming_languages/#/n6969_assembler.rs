//! Hello-world snippet for `ProgrammingLanguageName::N6969Assembler`.
//!
//! Imported from leachim6/hello-world (`#/6969 Assembler` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/6969%20Assembler>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N6969Assembler => "6969 Assembler",
    versions: [N6969Assembler => "6969 Assembler"],
    extensions: [],
}

const N6969_ASSEMBLER: &str = concat!(r"MOV C*::", output_for_hello_world!());

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N6969Assembler,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N6969Assembler,
            syntax: N6969_ASSEMBLER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N6969_ASSEMBLER,
                    [(
                        TokenKind::Plain,
                        concat!(r"MOV C*::", output_for_hello_world!()),
                    )],
                )
            },
        }],
    };

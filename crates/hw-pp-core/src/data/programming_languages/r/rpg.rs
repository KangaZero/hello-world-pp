//! Hello-world snippet for `ProgrammingLanguageName::Rpg`.
//!
//! Imported from leachim6/hello-world (`r/RPG.rpgle` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/RPG.rpgle>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rpg => "RPG",
    versions: [Rpg => "RPG"],
    extensions: [Rpgle => "rpgle"],
}

const RPG: &str = concat!(
    r"**free
dsply '",
    output_for_hello_world!(),
    r"';
return;"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rpg,
        extensions: &[FileExtension::Rpgle],
        categories: &[Compiled, Historical],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rpg,
            syntax: RPG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RPG,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"**free
dsply '",
                            output_for_hello_world!(),
                            r"';
return;"
                        ),
                    )],
                )
            },
        }],
    };

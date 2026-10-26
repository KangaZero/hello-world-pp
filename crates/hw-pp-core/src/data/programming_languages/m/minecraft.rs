//! Hello-world snippet for `ProgrammingLanguageName::Minecraft`.
//!
//! Imported from leachim6/hello-world (`m/Minecraft.mcfunction` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Minecraft.mcfunction>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Minecraft => "Minecraft",
    versions: [Minecraft => "Minecraft"],
    extensions: [],
}

const MINECRAFT: &str = concat!(r#"tellraw @p ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Minecraft,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Minecraft,
            syntax: MINECRAFT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MINECRAFT,
                    [(
                        TokenKind::Plain,
                        concat!(r#"tellraw @p ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };

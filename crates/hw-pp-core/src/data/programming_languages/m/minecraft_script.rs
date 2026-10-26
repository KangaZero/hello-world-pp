//! Hello-world snippet for `ProgrammingLanguageName::MinecraftScript`.
//!
//! Imported from leachim6/hello-world (`m/Minecraft Script.mcscript` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Minecraft%20Script.mcscript>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: MinecraftScript => "Minecraft Script",
    versions: [MinecraftScript => "Minecraft Script"],
    extensions: [Mcscript => "mcscript"],
}

const MINECRAFT_SCRIPT: &str = concat!(r#"log(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MinecraftScript,
        extensions: &[FileExtension::Mcscript],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MinecraftScript,
            syntax: MINECRAFT_SCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MINECRAFT_SCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(r#"log(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };

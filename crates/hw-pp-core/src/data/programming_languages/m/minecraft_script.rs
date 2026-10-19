//! Hello-world snippet for `ProgrammingLanguageName::MinecraftScript`.
//!
//! Imported from leachim6/hello-world (`m/Minecraft Script.mcscript` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: MinecraftScript => "Minecraft Script",
    versions: [MinecraftScript => "Minecraft Script"],
    extensions: [Mcscript => "mcscript"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MinecraftScript,
        extensions: &[FileExtension::Mcscript],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MinecraftScript,
            syntax: concat!(r#"log(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"log(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };

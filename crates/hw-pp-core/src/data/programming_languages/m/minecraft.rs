//! Hello-world snippet for `ProgrammingLanguageName::Minecraft`.
//!
//! Imported from leachim6/hello-world (`m/Minecraft.mcfunction` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Minecraft => "Minecraft",
    versions: [Minecraft => "Minecraft"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Minecraft,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Minecraft,
            syntax: concat!(r#"tellraw @p ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"tellraw @p ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

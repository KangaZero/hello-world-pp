//! Hello-world snippet for `ProgrammingLanguageName::Wolframlanguage`.
//!
//! Imported from leachim6/hello-world (`w/WolframLanguage.wl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/WolframLanguage.wl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scientific,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Wolframlanguage => "WolframLanguage",
    versions: [Wolframlanguage => "WolframLanguage"],
    extensions: [Wl => "wl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wolframlanguage,
        extensions: &[FileExtension::Wl],
        categories: &[Scientific],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wolframlanguage,
            syntax: concat!(r#"Print[""#, output_for_hello_world!(), r#""];"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Print[""#, output_for_hello_world!(), r#""];"#),
            )],
        }],
    };

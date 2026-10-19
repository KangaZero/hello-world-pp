//! Hello-world snippet for `ProgrammingLanguageName::Wolframlanguage`.
//!
//! Imported from leachim6/hello-world (`w/WolframLanguage.wl` at `a152253`),
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
    language: Wolframlanguage => "WolframLanguage",
    versions: [Wolframlanguage => "WolframLanguage"],
    extensions: [Wl => "wl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wolframlanguage,
        extensions: &[FileExtension::Wl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wolframlanguage,
            syntax: concat!(r#"Print[""#, output_for_hello_world!(), r#""];"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Print[""#, output_for_hello_world!(), r#""];"#),
            )],
        }],
    };

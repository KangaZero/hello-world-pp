//! Hello-world snippet for `ProgrammingLanguageName::CharcoalVerboseMode`.
//!
//! Imported from leachim6/hello-world (`c/Charcoal verbose mode.cl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Charcoal%20verbose%20mode.cl>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: CharcoalVerboseMode => "Charcoal verbose mode",
    versions: [CharcoalVerboseMode => "Charcoal verbose mode"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CharcoalVerboseMode,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CharcoalVerboseMode,
            syntax: concat!(r#"Print(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Print(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };

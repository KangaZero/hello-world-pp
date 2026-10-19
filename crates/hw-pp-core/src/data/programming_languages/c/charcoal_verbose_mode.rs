//! Hello-world snippet for `ProgrammingLanguageName::CharcoalVerboseMode`.
//!
//! Imported from leachim6/hello-world (`c/Charcoal verbose mode.cl` at `a152253`),
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
    language: CharcoalVerboseMode => "Charcoal verbose mode",
    versions: [CharcoalVerboseMode => "Charcoal verbose mode"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CharcoalVerboseMode,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CharcoalVerboseMode,
            syntax: concat!(r#"Print(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Print(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };

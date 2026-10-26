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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: CharcoalVerboseMode => "Charcoal verbose mode",
    versions: [CharcoalVerboseMode => "Charcoal verbose mode"],
    extensions: [],
}

const CHARCOAL_VERBOSE_MODE: &str = concat!(r#"Print(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CharcoalVerboseMode,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CharcoalVerboseMode,
            syntax: CHARCOAL_VERBOSE_MODE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CHARCOAL_VERBOSE_MODE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Print(""#, output_for_hello_world!(), r#"");"#),
                    )],
                )
            },
        }],
    };

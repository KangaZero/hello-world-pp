//! Hello-world snippet for `ProgrammingLanguageName::Lil`.
//!
//! Imported from leachim6/hello-world (PR #1602 `l/Lil.lil`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1602>
//! - <https://beyondloom.com/decker/lil.html> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Array, Declarative, Functional, Imperative};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lil => "Lil",
    versions: [Lil => "Lil"],
    extensions: [Lil => "lil"],
}

const LIL: &str = concat!(r#"print[""#, output_for_hello_world!(), r#""]"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lil,
        extensions: &[FileExtension::Lil],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, Functional, Declarative, Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lil,
            syntax: LIL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LIL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print[""#, output_for_hello_world!(), r#""]"#),
                    )],
                )
            },
        }],
    };

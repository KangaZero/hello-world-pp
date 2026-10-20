//! Hello-world snippet for `ProgrammingLanguageName::Foxpro`.
//!
//! Imported from leachim6/hello-world (`f/FoxPro.prg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/FoxPro.prg>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::XBase, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Foxpro => "FoxPro",
    versions: [Foxpro => "FoxPro"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Foxpro,
        extensions: &[],
        categories: &[Historical],
        families: &[XBase],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Foxpro,
            syntax: concat!(r#"? ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"? ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

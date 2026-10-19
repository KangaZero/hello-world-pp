//! Hello-world snippet for `ProgrammingLanguageName::Rebmu`.
//!
//! Imported from leachim6/hello-world (`r/REBMU.reb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/REBMU.reb>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Rebmu => "REBMU",
    versions: [Rebmu => "REBMU"],
    extensions: [Reb => "reb"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rebmu,
        extensions: &[FileExtension::Reb],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rebmu,
            syntax: concat!(r#"rebmu [p""#, output_for_hello_world!(), r#""]"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"rebmu [p""#, output_for_hello_world!(), r#""]"#),
            )],
        }],
    };

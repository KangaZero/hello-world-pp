//! Hello-world snippet for `ProgrammingLanguageName::Yoptascript`.
//!
//! Imported from leachim6/hello-world (`y/YoptaScript` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/y/YoptaScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Yoptascript => "YoptaScript",
    versions: [Yoptascript => "YoptaScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yoptascript,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yoptascript,
            syntax: concat!(r#"ксива.малява(""#, output_for_hello_world!(), r#"") нах"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"ксива.малява(""#, output_for_hello_world!(), r#"") нах"#),
            )],
        }],
    };

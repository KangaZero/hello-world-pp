//! Hello-world snippet for `ProgrammingLanguageName::Factor`.
//!
//! Imported from leachim6/hello-world (`f/Factor.factor` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Factor.factor>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Factor => "Factor",
    versions: [Factor => "Factor"],
    extensions: [Factor => "factor"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Factor,
        extensions: &[FileExtension::Factor],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Factor,
            syntax: concat!(
                r#"USING: io ;
""#,
                output_for_hello_world!(),
                r#"" print
"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"USING: io ;
""#,
                    output_for_hello_world!(),
                    r#"" print
"#
                ),
            )],
        }],
    };

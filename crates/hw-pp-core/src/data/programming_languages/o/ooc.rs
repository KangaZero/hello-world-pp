//! Hello-world snippet for `ProgrammingLanguageName::Ooc`.
//!
//! Imported from leachim6/hello-world (`o/Ooc.ooc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Ooc.ooc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ooc => "Ooc",
    versions: [Ooc => "Ooc"],
    extensions: [Ooc => "ooc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ooc,
        extensions: &[FileExtension::Ooc],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ooc,
            syntax: concat!(
                r#"main: func {
    ""#,
                output_for_hello_world!(),
                r#"" println()
}
"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"main: func {
    ""#,
                    output_for_hello_world!(),
                    r#"" println()
}
"#
                ),
            )],
        }],
    };

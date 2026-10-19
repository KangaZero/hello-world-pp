//! Hello-world snippet for `ProgrammingLanguageName::Ooc`.
//!
//! Imported from leachim6/hello-world (`o/Ooc.ooc` at `a152253`),
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
    language: Ooc => "Ooc",
    versions: [Ooc => "Ooc"],
    extensions: [Ooc => "ooc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ooc,
        extensions: &[FileExtension::Ooc],
        categories: &[],
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

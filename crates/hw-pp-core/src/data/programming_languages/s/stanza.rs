//! Hello-world snippet for `ProgrammingLanguageName::Stanza`.
//!
//! Imported from leachim6/hello-world (`s/Stanza.stanza` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Stanza.stanza>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Stanza => "Stanza",
    versions: [Stanza => "Stanza"],
    extensions: [Stanza => "stanza"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stanza,
        extensions: &[FileExtension::Stanza],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stanza,
            syntax: concat!(
                r#"println(""#,
                output_for_hello_world!(),
                r#"")
"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"println(""#,
                    output_for_hello_world!(),
                    r#"")
"#
                ),
            )],
        }],
    };

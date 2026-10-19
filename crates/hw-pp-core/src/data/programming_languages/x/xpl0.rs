//! Hello-world snippet for `ProgrammingLanguageName::Xpl0`.
//!
//! Imported from leachim6/hello-world (`x/XPL0.xpl` at `a152253`),
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
    language: Xpl0 => "XPL0",
    versions: [Xpl0 => "XPL0"],
    extensions: [Xpl => "xpl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xpl0,
        extensions: &[FileExtension::Xpl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xpl0,
            syntax: concat!(
                r#"code Text=12;
Text(0, ""#,
                output_for_hello_world!(),
                r#"")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"code Text=12;
Text(0, ""#,
                    output_for_hello_world!(),
                    r#"")"#
                ),
            )],
        }],
    };

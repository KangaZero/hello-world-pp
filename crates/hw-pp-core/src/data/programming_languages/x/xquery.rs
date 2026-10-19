//! Hello-world snippet for `ProgrammingLanguageName::Xquery`.
//!
//! Imported from leachim6/hello-world (`x/XQuery.xq` at `a152253`),
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
    language: Xquery => "XQuery",
    versions: [Xquery => "XQuery"],
    extensions: [Xq => "xq"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xquery,
        extensions: &[FileExtension::Xq],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xquery,
            syntax: concat!(
                r#"let $hello := ""#,
                output_for_hello_world!(),
                r#""
return $hello"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"let $hello := ""#,
                    output_for_hello_world!(),
                    r#""
return $hello"#
                ),
            )],
        }],
    };

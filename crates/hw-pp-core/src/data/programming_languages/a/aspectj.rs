//! Hello-world snippet for `ProgrammingLanguageName::Aspectj`.
//!
//! Imported from leachim6/hello-world (`a/Aspectj.aj` at `a152253`),
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
    language: Aspectj => "Aspectj",
    versions: [Aspectj => "Aspectj"],
    extensions: [Aj => "aj"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aspectj,
        extensions: &[FileExtension::Aj],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aspectj,
            syntax: concat!(
                r#"System.out.println(""#,
                output_for_hello_world!(),
                r#"");"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"System.out.println(""#,
                    output_for_hello_world!(),
                    r#"");"#
                ),
            )],
        }],
    };

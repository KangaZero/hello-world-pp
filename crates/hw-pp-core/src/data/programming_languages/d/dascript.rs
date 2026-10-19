//! Hello-world snippet for `ProgrammingLanguageName::Dascript`.
//!
//! Imported from leachim6/hello-world (`d/daScript.das` at `a152253`),
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
    language: Dascript => "daScript",
    versions: [Dascript => "daScript"],
    extensions: [Das => "das"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dascript,
        extensions: &[FileExtension::Das],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dascript,
            syntax: concat!(
                r#"[export]
def main
	print(""#,
                output_for_hello_world!(),
                r#"\n")"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"[export]
def main
	print(""#,
                    output_for_hello_world!(),
                    r#"\n")"#
                ),
            )],
        }],
    };

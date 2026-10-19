//! Hello-world snippet for `ProgrammingLanguageName::JsxDena`.
//!
//! Imported from leachim6/hello-world (`j/JSX(DeNA).jsx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/JSX%28DeNA%29.jsx>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: JsxDena => "JSX(DeNA)",
    versions: [JsxDena => "JSX(DeNA)"],
    extensions: [Jsx => "jsx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JsxDena,
        extensions: &[FileExtension::Jsx],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::JsxDena,
            syntax: concat!(
                r#" class _Main {
     static function main(args : string[]) : void {
         log ""#,
                output_for_hello_world!(),
                r#"";
     }
 }"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#" class _Main {
     static function main(args : string[]) : void {
         log ""#,
                    output_for_hello_world!(),
                    r#"";
     }
 }"#
                ),
            )],
        }],
    };

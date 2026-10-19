//! Hello-world snippet for `ProgrammingLanguageName::Loment`.
//!
//! Imported from leachim6/hello-world (PR #1857 `l/Loment.lomt`),
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
    language: Loment => "Loment",
    versions: [Loment => "Loment"],
    extensions: [Lomt => "lomt"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Loment,
        extensions: &[FileExtension::Lomt],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Loment,
            syntax: concat!(
                r#"module hello

fn _start() {
    let s: str = ""#,
                output_for_hello_world!(),
                r#"\n";
    syscall4(1, 1, str_ptr(s) as u64, str_len(s) as u64);
    syscall4(60, 0, 0, 0);
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module hello

fn _start() {
    let s: str = ""#,
                    output_for_hello_world!(),
                    r#"\n";
    syscall4(1, 1, str_ptr(s) as u64, str_len(s) as u64);
    syscall4(60, 0, 0, 0);
}"#
                ),
            )],
        }],
    };

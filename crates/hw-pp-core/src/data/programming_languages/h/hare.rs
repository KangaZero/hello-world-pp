//! Hello-world snippet for `ProgrammingLanguageName::Hare`.
//!
//! Imported from leachim6/hello-world (`h/Hare.ha` at `a152253`),
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
    language: Hare => "Hare",
    versions: [Hare => "Hare"],
    extensions: [Ha => "ha"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hare,
        extensions: &[FileExtension::Ha],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hare,
            syntax: concat!(
                r#"use fmt;

export fn main() void = {
	fmt::println(""#,
                output_for_hello_world!(),
                r#"")!;
};"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"use fmt;

export fn main() void = {
	fmt::println(""#,
                    output_for_hello_world!(),
                    r#"")!;
};"#
                ),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::TypeScript`.
//!
//! Sources:
//! - <https://www.typescriptlang.org/docs/handbook/typescript-in-5-minutes.html>
//! - <https://www.typescriptlang.org/docs/handbook/2/basic-types.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Scripting},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::JavaScript,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: TypeScript => "TypeScript",
    versions: [TypeScript5 => "TypeScript 5"],
    extensions: [Ts => "ts"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TypeScript,
        extensions: &[FileExtension::Ts],
        categories: &[Scripting, Compiled],
        families: &[JavaScript],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TypeScript5,
            syntax: concat!(
                r#"const greeting: string = ""#,
                output_for_hello_world!(),
                r#"";
console.log(greeting);"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "const"),
                (TokenKind::Plain, " greeting"),
                (TokenKind::Punctuation, ":"),
                (TokenKind::Plain, " "),
                (TokenKind::Type, "string"),
                (TokenKind::Plain, " "),
                (TokenKind::Punctuation, "="),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Punctuation, ";"),
                (TokenKind::Plain, "\nconsole"),
                (TokenKind::Punctuation, "."),
                (TokenKind::Function, "log"),
                (TokenKind::Punctuation, "("),
                (TokenKind::Plain, "greeting"),
                (TokenKind::Punctuation, ");"),
            ],
        }],
    };

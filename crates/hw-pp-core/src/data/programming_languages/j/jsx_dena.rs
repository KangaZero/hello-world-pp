//! Hello-world snippet for `ProgrammingLanguageName::JsxDena`.
//!
//! Imported from leachim6/hello-world (`j/JSX(DeNA).jsx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/JSX%28DeNA%29.jsx>
//! - <https://jsx.github.io/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: JsxDena => "JSX(DeNA)",
    versions: [JsxDena => "JSX(DeNA)"],
    extensions: [Jsx => "jsx"],
}

const JSX_DENA: &str = concat!(
    r#" class _Main {
     static function main(args : string[]) : void {
         log ""#,
    output_for_hello_world!(),
    r#"";
     }
 }"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JsxDena,
        extensions: &[FileExtension::Jsx],
        categories: &[],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::JsxDena,
            syntax: JSX_DENA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JSX_DENA,
                    [
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"class"),
                        (TokenKind::Plain, r" _Main "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
     ",
                        ),
                        (TokenKind::Keyword, r"static"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"function"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"args "),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Type, r"string"),
                        (TokenKind::Punctuation, r"[])"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
         ",
                        ),
                        (TokenKind::Function, r"log"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
     ",
                        ),
                        (TokenKind::Punctuation, r"}"),
                        (
                            TokenKind::Plain,
                            r"
 ",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };

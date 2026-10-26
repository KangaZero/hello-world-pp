//! Hello-world snippet for `ProgrammingLanguageName::Haxe`.
//!
//! Imported from leachim6/hello-world (`h/Haxe.hx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Haxe.hx>
//! - <https://www.wikidata.org/wiki/Q285886> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Generic, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Haxe => "Haxe",
    versions: [Haxe => "Haxe"],
    extensions: [Hx => "hx"],
}

const HAXE: &str = concat!(
    r#"class HelloWorld {
    static function main() {
        trace(""#,
    output_for_hello_world!(),
    r#"");
    }
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Haxe,
        extensions: &[FileExtension::Hx],
        categories: &[Compiled],
        families: &[],
        paradigms: &[ObjectOriented, Functional, Generic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Haxe,
            syntax: HAXE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HAXE,
                    [
                        (TokenKind::Keyword, r"class"),
                        (TokenKind::Plain, r" HelloWorld "),
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
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Function, r"trace"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
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

//! Hello-world snippet for `ProgrammingLanguageName::Angelscript`.
//!
//! Imported from leachim6/hello-world (`a/AngelScript` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/AngelScript>
//! - <https://www.wikidata.org/wiki/Q2366> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Imperative, ObjectOriented, Procedural,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Angelscript => "AngelScript",
    versions: [Angelscript => "AngelScript"],
    extensions: [],
}

const ANGELSCRIPT: &str = concat!(
    r#"void main() { print(""#,
    output_for_hello_world!(),
    r#"\n"); }"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Angelscript,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, Procedural, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Angelscript,
            syntax: ANGELSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ANGELSCRIPT,
                    [
                        (TokenKind::Keyword, r"void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::C2`.
//!
//! Imported from leachim6/hello-world (`c/C--` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/C-->

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: C2 => "C--",
    versions: [C2 => "C--"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::C2,
        extensions: &[],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::C2,
            syntax: concat!(
                r#"target byteorder little;
import puts;
export main;

section "data"{
    s:bits8[] ""#,
                output_for_hello_world!(),
                r#"\0";
}

foreign "C" main(){
    foreign "C" puts("address"s);
    foreign "C" return(0);
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, r"target byteorder little"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"puts"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"export"),
                (TokenKind::Plain, r" main"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"

section ",
                ),
                (TokenKind::String, r#""data""#),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    s",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r"bits8"),
                (TokenKind::Punctuation, r"[]"),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"\0""#),
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

foreign ",
                ),
                (TokenKind::String, r#""C""#),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"main"),
                (TokenKind::Punctuation, r"(){"),
                (
                    TokenKind::Plain,
                    r"
    foreign ",
                ),
                (TokenKind::String, r#""C""#),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"puts"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::String, r#""address""#),
                (TokenKind::Plain, r"s"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    foreign ",
                ),
                (TokenKind::String, r#""C""#),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"return"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Number, r"0"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };

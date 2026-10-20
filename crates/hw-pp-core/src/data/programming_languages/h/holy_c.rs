//! Hello-world snippet for `ProgrammingLanguageName::HolyC`.
//!
//! Imported from leachim6/hello-world (`h/Holy C.HC` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Holy%20C.HC>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: HolyC => "Holy C",
    versions: [HolyC => "Holy C"],
    extensions: [Hc => "HC"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HolyC,
        extensions: &[FileExtension::Hc],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HolyC,
            syntax: concat!(
                r#"U0 main() {
    print(""#,
                output_for_hello_world!(),
                r#"\n");
    return 0;
}

main();"#
            ),
            tokenized_syntax: &[
                (TokenKind::Plain, r"U0 "),
                (TokenKind::Function, r"main"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Function, r"print"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"return"),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"0"),
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
                (TokenKind::Function, r"main"),
                (TokenKind::Punctuation, r"();"),
            ],
        }],
    };

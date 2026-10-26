//! Hello-world snippet for `ProgrammingLanguageName::Cotowali`.
//!
//! Imported from leachim6/hello-world (`c/Cotowali.li` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cotowali.li>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cotowali => "Cotowali",
    versions: [Cotowali => "Cotowali"],
    extensions: [Li => "li"],
}

const COTOWALI: &str = concat!(
    r#"fn helloworld(): string {
  return ""#,
    output_for_hello_world!(),
    r#""
}

helloworld()"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cotowali,
        extensions: &[FileExtension::Li],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cotowali,
            syntax: COTOWALI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COTOWALI,
                    [
                        (TokenKind::Keyword, r"fn"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"helloworld"),
                        (TokenKind::Punctuation, r"():"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Type, r"string"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
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
                        (TokenKind::Function, r"helloworld"),
                        (TokenKind::Punctuation, r"()"),
                    ],
                )
            },
        }],
    };

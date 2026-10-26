//! Hello-world snippet for `ProgrammingLanguageName::Tengo`.
//!
//! Imported from leachim6/hello-world (`t/Tengo.tengo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Tengo.tengo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Tengo => "Tengo",
    versions: [Tengo => "Tengo"],
    extensions: [Tengo => "tengo"],
}

const TENGO: &str = concat!(
    r#"fmt := import("fmt")
fmt.print(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tengo,
        extensions: &[FileExtension::Tengo],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tengo,
            syntax: TENGO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TENGO,
                    [
                        (TokenKind::Plain, r"fmt "),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"= "),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""fmt""#),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
fmt",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };

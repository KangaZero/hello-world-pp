//! Hello-world snippet for `ProgrammingLanguageName::Myrddin`.
//!
//! Imported from leachim6/hello-world (`m/Myrddin.myr` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Myrddin.myr>
//! - <https://myrlang.org/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Generic;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Myrddin => "Myrddin",
    versions: [Myrddin => "Myrddin"],
    extensions: [Myr => "myr"],
}

const MYRDDIN: &str = concat!(
    r#"use std

const main = {
    std.put(""#,
    output_for_hello_world!(),
    r#"\n")
}
"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Myrddin,
        extensions: &[FileExtension::Myr],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Generic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Myrddin,
            syntax: MYRDDIN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MYRDDIN,
                    [
                        (TokenKind::Keyword, r"use"),
                        (
                            TokenKind::Plain,
                            r" std

",
                        ),
                        (TokenKind::Keyword, r"const"),
                        (TokenKind::Plain, r" main = "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
    std",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"put"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, r")"),
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
                    ],
                )
            },
        }],
    };

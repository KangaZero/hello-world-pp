//! Hello-world snippet for `ProgrammingLanguageName::Vala`.
//!
//! Imported from leachim6/hello-world (`v/Vala.vala` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/Vala.vala>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::C, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Vala => "Vala",
    versions: [Vala => "Vala"],
    extensions: [Vala => "vala"],
}

const VALA: &str = concat!(
    r#"static void main (string[] args)
{
	stdout.printf (""#,
    output_for_hello_world!(),
    r#"\n");
}
"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vala,
        extensions: &[FileExtension::Vala],
        categories: &[Compiled],
        families: &[C],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vala,
            syntax: VALA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VALA,
                    [
                        (TokenKind::Keyword, r"static"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Type, r"string"),
                        (TokenKind::Punctuation, r"[]"),
                        (TokenKind::Plain, r" args"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
	stdout",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
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

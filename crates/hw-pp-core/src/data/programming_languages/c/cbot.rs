//! Hello-world snippet for `ProgrammingLanguageName::Cbot`.
//!
//! Imported from leachim6/hello-world (`c/Cbot.txt` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cbot.txt>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cbot => "Cbot",
    versions: [Cbot => "Cbot"],
    extensions: [Txt => "txt"],
}

const CBOT: &str = concat!(
    r#"extern void object::New()
{

message(""#,
    output_for_hello_world!(),
    r#"");

}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cbot,
        extensions: &[FileExtension::Txt],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cbot,
            syntax: CBOT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CBOT,
                    [
                        (TokenKind::Keyword, r"extern"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"object"),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Function, r"New"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Function, r"message"),
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
                    ],
                )
            },
        }],
    };

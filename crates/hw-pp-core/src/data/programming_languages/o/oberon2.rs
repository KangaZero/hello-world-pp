//! Hello-world snippet for `ProgrammingLanguageName::Oberon2`.
//!
//! Imported from leachim6/hello-world (`o/Oberon 2.obn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Oberon%202.obn>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{Modula, Oberon},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Oberon2 => "Oberon 2",
    versions: [Oberon2 => "Oberon 2"],
    extensions: [Obn => "obn"],
}

const OBERON2: &str = concat!(
    r#"MODULE HelloWorld;                     
IMPORT Out;
BEGIN
  Out.String( ""#,
    output_for_hello_world!(),
    r#"" );
  Out.Ln;
END HelloWorld."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Oberon2,
        extensions: &[FileExtension::Obn],
        categories: &[Compiled, Historical],
        families: &[Modula, Oberon],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Oberon2,
            syntax: OBERON2,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OBERON2,
                    [
                        (TokenKind::Keyword, r"MODULE"),
                        (TokenKind::Plain, r" HelloWorld"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"                     
",
                        ),
                        (TokenKind::Keyword, r"IMPORT"),
                        (TokenKind::Plain, r" Out"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"BEGIN"),
                        (
                            TokenKind::Plain,
                            r"
  Out",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"String"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
  Out",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"Ln"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Plain, r" HelloWorld"),
                        (TokenKind::Punctuation, r"."),
                    ],
                )
            },
        }],
    };

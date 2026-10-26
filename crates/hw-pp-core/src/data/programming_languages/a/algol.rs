//! Hello-world snippet for `ProgrammingLanguageName::Algol`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/ALGOL_60>
//! - <https://www.masswerk.at/algol60/report.htm>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Algol,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Algol => "ALGOL",
    versions: [Algol60 => "ALGOL 60"],
    extensions: [Alg => "alg"],
}

const ALGOL60: &str = concat!(
    r#"begin
    outstring(1, ""#,
    output_for_hello_world!(),
    r#"\n")
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Algol,
        extensions: &[FileExtension::Alg],
        categories: &[Compiled, Historical],
        families: &[Algol],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Algol60,
            syntax: ALGOL60,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ALGOL60,
                    [
                        (TokenKind::Keyword, "begin"),
                        (TokenKind::Plain, "\n    "),
                        (TokenKind::Function, "outstring"),
                        (TokenKind::Punctuation, "("),
                        (TokenKind::Number, "1"),
                        (TokenKind::Punctuation, ","),
                        (TokenKind::Plain, " "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), r#"\n""#),
                        ),
                        (TokenKind::Punctuation, ")"),
                        (TokenKind::Plain, "\n"),
                        (TokenKind::Keyword, "end"),
                    ],
                )
            },
        }],
    };

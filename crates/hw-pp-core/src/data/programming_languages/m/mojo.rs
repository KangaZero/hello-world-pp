//! Hello-world snippet for `ProgrammingLanguageName::Mojo`.
//!
//! Imported from leachim6/hello-world (`m/Mojo.🔥` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mojo.%F0%9F%94%A5>
//! - <https://www.wikidata.org/wiki/Q118152673> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Generic, Imperative};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Python, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Mojo => "Mojo",
    versions: [Mojo => "Mojo"],
    extensions: [U1f525 => "🔥"],
}

const MOJO: &str = concat!(
    r#"fn main():
  print(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mojo,
        extensions: &[FileExtension::U1f525],
        categories: &[Compiled],
        families: &[Python],
        paradigms: &[Imperative, Functional, Generic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mojo,
            syntax: MOJO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MOJO,
                    [
                        (TokenKind::Keyword, r"fn"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"():"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
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

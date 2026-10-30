//! Hello-world snippet for `ProgrammingLanguageName::Picat`.
//!
//! Imported from leachim6/hello-world (`p/PiCat.pi` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PiCat.pi>
//! - <http://picat-lang.org/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Constraint, Functional, Imperative, Logic,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Picat => "PiCat",
    versions: [Picat => "PiCat"],
    extensions: [Pi => "pi"],
}

const PICAT: &str = concat!(
    r#"main =>
  println(""#,
    output_for_hello_world!(),
    r#"")."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Picat,
        extensions: &[FileExtension::Pi],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, Functional, Logic, Constraint],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Picat,
            syntax: PICAT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PICAT,
                    [
                        (TokenKind::Keyword, r"main"),
                        (
                            TokenKind::Plain,
                            r" =>
  ",
                        ),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")."),
                    ],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Make`.
//!
//! Imported from leachim6/hello-world (`m/Make.makefile` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Make.makefile>
//! - <https://en.wikipedia.org/wiki/Make_(software)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Declarative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Make => "Make",
    versions: [Make => "Make"],
    extensions: [Makefile => "makefile"],
}

const MAKE: &str = concat!(
    r#"$(info ""#,
    output_for_hello_world!(),
    r#"")
all:"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Make,
        extensions: &[FileExtension::Makefile],
        categories: &[Configuration],
        families: &[],
        paradigms: &[Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Make,
            syntax: MAKE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MAKE,
                    [
                        (TokenKind::Keyword, r"$("),
                        (TokenKind::Function, r"info"),
                        (
                            TokenKind::Plain,
                            concat!(r#" ""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Keyword, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Function, r"all"),
                        (TokenKind::Keyword, r":"),
                    ],
                )
            },
        }],
    };

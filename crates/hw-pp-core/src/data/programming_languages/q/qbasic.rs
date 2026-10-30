//! Hello-world snippet for `ProgrammingLanguageName::Qbasic`.
//!
//! Imported from leachim6/hello-world (`q/QBasic.bas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/QBasic.bas>
//! - <https://www.wikidata.org/wiki/Q381247> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Historical, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Basic, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Qbasic => "QBasic",
    versions: [Qbasic => "QBasic"],
    extensions: [],
}

const QBASIC: &str = concat!(
    r#"
PRINT ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qbasic,
        extensions: &[],
        categories: &[Historical],
        families: &[Basic],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qbasic,
            syntax: QBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QBASIC,
                    [
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"PRINT"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };

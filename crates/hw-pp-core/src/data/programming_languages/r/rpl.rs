//! Hello-world snippet for `ProgrammingLanguageName::Rpl`.
//!
//! Imported from the Hello World Collection (`files/RPL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/RPL.txt>
//! - <http://helloworldcollection.de/#RPL>
//! - <https://www.wikidata.org/wiki/Q960361> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    ObjectOriented, StackBased, Structured,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Lisp,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Rpl => "RPL",
    versions: [Rpl => "RPL"],
    extensions: [],
}

const RPL: &str = concat!(
    output_for_hello_world!(),
    r#" in RPL for the HP-28, HP-48, HP-49 and HP-50 series pocket calculators. No comments possible.

<<
    ""#,
    output_for_hello_world!(),
    r#""
    1 DISP
    60 FREEZE
>>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rpl,
        extensions: &[],
        categories: &[],
        families: &[Lisp],
        paradigms: &[Structured, ObjectOriented, StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rpl,
            syntax: RPL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RPL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            output_for_hello_world!(),
                            r#" in RPL for the HP-28, HP-48, HP-49 and HP-50 series pocket calculators. No comments possible.

<<
    ""#,
                            output_for_hello_world!(),
                            r#""
    1 DISP
    60 FREEZE
>>"#
                        ),
                    )],
                )
            },
        }],
    };

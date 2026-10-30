//! Hello-world snippet for `ProgrammingLanguageName::Freebasic`.
//!
//! Imported from the Hello World Collection (`files/FreeBASIC.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/FreeBASIC.txt>
//! - <http://helloworldcollection.de/#FreeBASIC>
//! - <https://www.wikidata.org/wiki/Q195929> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{ObjectOriented, Procedural};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Freebasic => "FreeBASIC",
    versions: [Freebasic => "FreeBASIC"],
    extensions: [],
}

const FREEBASIC: &str = concat!(
    r"'",
    output_for_hello_world!(),
    r#" in FreeBASIC

print ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Freebasic,
        extensions: &[],
        categories: &[],
        families: &[Basic],
        paradigms: &[Procedural, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Freebasic,
            syntax: FREEBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FREEBASIC,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"'", output_for_hello_world!(), r" in FreeBASIC"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"print"),
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

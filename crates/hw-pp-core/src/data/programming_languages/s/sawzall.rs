//! Hello-world snippet for `ProgrammingLanguageName::Sawzall`.
//!
//! Imported from the Hello World Collection (`files/Sawzall.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Sawzall.htm>
//! - <http://helloworldcollection.de/#Sawzall>
//! - <https://www.wikidata.org/wiki/Q970424> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Sawzall => "Sawzall",
    versions: [Sawzall => "Sawzall"],
    extensions: [],
}

const SAWZALL: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r#" in Sawzall

emit stdout <- ""#,
    output_for_hello_world!(),
    r#"";"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sawzall,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sawzall,
            syntax: SAWZALL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SAWZALL,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"# ", output_for_hello_world!(), r" in Sawzall"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

emit stdout <- ",
                        ),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r";"),
                    ],
                )
            },
        }],
    };

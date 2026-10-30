//! Hello-world snippet for `ProgrammingLanguageName::Apl`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/APL_(programming_language)>
//! - <https://tryapl.org/>
//! - <https://www.wikidata.org/wiki/Q296187> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Array, Functional, Structured};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Apl,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Apl => "APL",
    versions: [Apl => "APL"],
    extensions: [Apl => "apl"],
}

const APL: &str = concat!(r"'", output_for_hello_world!(), r"'");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Apl,
        extensions: &[FileExtension::Apl],
        categories: &[Compiled, Historical],
        families: &[Apl],
        paradigms: &[Structured, Functional, Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Apl,
            syntax: APL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    APL,
                    [(
                        TokenKind::String,
                        concat!("'", output_for_hello_world!(), "'"),
                    )],
                )
            },
        }],
    };

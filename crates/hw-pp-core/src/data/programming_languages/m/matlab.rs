//! Hello-world snippet for `ProgrammingLanguageName::Matlab`.
//!
//! Sources:
//! - <https://en.wikipedia.org/wiki/MATLAB>
//! - <https://www.wikidata.org/wiki/Q37805571> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Array, Functional, Imperative, ObjectOriented, Procedural,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Matlab => "MATLAB",
    versions: [Matlab => "MATLAB"],
    extensions: [M => "m"],
}

const MATLAB: &str = concat!(r"disp('", output_for_hello_world!(), r"')");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Matlab,
        extensions: &[FileExtension::M],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, Procedural, ObjectOriented, Functional, Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Matlab,
            syntax: MATLAB,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MATLAB,
                    [
                        (TokenKind::Keyword, "disp"),
                        (TokenKind::Punctuation, "("),
                        (
                            TokenKind::String,
                            concat!("'", output_for_hello_world!(), "'"),
                        ),
                        (TokenKind::Punctuation, ")"),
                    ],
                )
            },
        }],
    };

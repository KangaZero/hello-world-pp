//! Hello-world snippet for `ProgrammingLanguageName::Octave`.
//!
//! Imported from leachim6/hello-world (`o/Octave.m` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Octave.m>
//! - <https://www.wikidata.org/wiki/Q223679> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Array;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scientific, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Octave => "Octave",
    versions: [Octave => "Octave"],
    extensions: [],
}

const OCTAVE: &str = concat!(r#"printf(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Octave,
        extensions: &[],
        categories: &[Scientific],
        families: &[],
        paradigms: &[Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Octave,
            syntax: OCTAVE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OCTAVE,
                    [
                        (TokenKind::Plain, r"printf"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r";"),
                    ],
                )
            },
        }],
    };

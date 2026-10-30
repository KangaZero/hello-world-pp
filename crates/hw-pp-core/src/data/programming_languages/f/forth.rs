//! Hello-world snippet for `ProgrammingLanguageName::Forth`.
//!
//! Imported from leachim6/hello-world (`f/Forth.fth` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Forth.fth>
//! - <https://www.wikidata.org/wiki/Q275472> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Concatenative, Procedural, Reflective, StackBased,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Forth, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Forth => "Forth",
    versions: [Forth => "Forth"],
    extensions: [Fth => "fth"],
}

const FORTH: &str = concat!(r".( ", output_for_hello_world!(), r")");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Forth,
        extensions: &[FileExtension::Fth],
        categories: &[Scripting],
        families: &[Forth],
        paradigms: &[Procedural, Reflective, StackBased, Concatenative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Forth,
            syntax: FORTH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FORTH,
                    [(
                        TokenKind::Plain,
                        concat!(r".( ", output_for_hello_world!(), r")"),
                    )],
                )
            },
        }],
    };

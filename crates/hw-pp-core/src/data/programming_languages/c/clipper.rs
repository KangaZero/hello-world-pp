//! Hello-world snippet for `ProgrammingLanguageName::Clipper`.
//!
//! Imported from leachim6/hello-world (`c/Clipper.prg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Clipper.prg>
//! - <https://en.wikipedia.org/wiki/Clipper_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::XBase,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Clipper => "Clipper",
    versions: [Clipper => "Clipper"],
    extensions: [Prg => "prg"],
}

const CLIPPER: &str = concat!(r#"? ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clipper,
        extensions: &[FileExtension::Prg],
        categories: &[Compiled, Historical],
        families: &[XBase],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clipper,
            syntax: CLIPPER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CLIPPER,
                    [(
                        TokenKind::Plain,
                        concat!(r#"? ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };

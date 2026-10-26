//! Hello-world snippet for `ProgrammingLanguageName::Blitzmax`.
//!
//! Imported from leachim6/hello-world (PR #1524 `b/BlitzMax.bmx`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1524>
//! - <https://www.wikidata.org/wiki/Q143419> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Blitzmax => "BlitzMax",
    versions: [Blitzmax => "BlitzMax"],
    extensions: [Bmx => "bmx"],
}

const BLITZMAX: &str = concat!(r#"Print ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blitzmax,
        extensions: &[FileExtension::Bmx],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Imperative, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blitzmax,
            syntax: BLITZMAX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BLITZMAX,
                    [
                        (TokenKind::Keyword, r"Print"),
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

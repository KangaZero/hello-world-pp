//! Hello-world snippet for `ProgrammingLanguageName::Netrexx`.
//!
//! Imported from leachim6/hello-world (`n/NetRexx.nrx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/NetRexx.nrx>
//! - <https://www.wikidata.org/wiki/Q6998311> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    ObjectOriented, Procedural, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Rexx, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Netrexx => "NetRexx",
    versions: [Netrexx => "NetRexx"],
    extensions: [Nrx => "nrx"],
}

const NETREXX: &str = concat!(r"say '", output_for_hello_world!(), r"'");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Netrexx,
        extensions: &[FileExtension::Nrx],
        categories: &[Compiled],
        families: &[Rexx],
        paradigms: &[Procedural, Structured, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Netrexx,
            syntax: NETREXX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NETREXX,
                    [
                        (TokenKind::Keyword, r"say"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                    ],
                )
            },
        }],
    };

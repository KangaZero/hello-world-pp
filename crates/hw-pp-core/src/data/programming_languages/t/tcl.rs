//! Hello-world snippet for `ProgrammingLanguageName::Tcl`.
//!
//! Imported from leachim6/hello-world (`t/Tcl.tcl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Tcl.tcl>
//! - <https://www.wikidata.org/wiki/Q5288> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    EventDriven, Functional, Imperative, ObjectOriented, Procedural,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Tcl, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Tcl => "Tcl",
    versions: [Tcl => "Tcl"],
    extensions: [Tcl => "tcl"],
}

const TCL: &str = concat!(r#"puts ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tcl,
        extensions: &[FileExtension::Tcl],
        categories: &[Scripting],
        families: &[Tcl],
        paradigms: &[
            Imperative,
            Procedural,
            ObjectOriented,
            Functional,
            EventDriven,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tcl,
            syntax: TCL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TCL,
                    [
                        (TokenKind::Keyword, r"puts"),
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

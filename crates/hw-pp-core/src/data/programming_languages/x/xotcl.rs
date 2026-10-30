//! Hello-world snippet for `ProgrammingLanguageName::Xotcl`.
//!
//! Imported from leachim6/hello-world (`x/XOTcl.xotcl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XOTcl.xotcl>
//! - <https://www.wikidata.org/wiki/Q4053333> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    EventDriven, Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Tcl, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Xotcl => "XOTcl",
    versions: [Xotcl => "XOTcl"],
    extensions: [Xotcl => "xotcl"],
}

const XOTCL: &str = concat!(r#"puts ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xotcl,
        extensions: &[FileExtension::Xotcl],
        categories: &[Scripting],
        families: &[Tcl],
        paradigms: &[Imperative, ObjectOriented, Functional, EventDriven],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xotcl,
            syntax: XOTCL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XOTCL,
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

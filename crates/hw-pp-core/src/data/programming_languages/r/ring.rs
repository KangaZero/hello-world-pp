//! Hello-world snippet for `ProgrammingLanguageName::Ring`.
//!
//! Imported from leachim6/hello-world (`r/Ring.ring` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Ring.ring>
//! - <https://www.wikidata.org/wiki/Q25462748> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Declarative, Functional, Imperative, ObjectOriented, Procedural,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Ring => "Ring",
    versions: [Ring => "Ring"],
    extensions: [Ring => "ring"],
}

const RING: &str = concat!(r#"see ""#, output_for_hello_world!(), r#"" "#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ring,
        extensions: &[FileExtension::Ring],
        categories: &[Scripting],
        families: &[],
        paradigms: &[
            Imperative,
            Procedural,
            ObjectOriented,
            Functional,
            Declarative,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ring,
            syntax: RING,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RING,
                    [
                        (TokenKind::Plain, r"see "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r" "),
                    ],
                )
            },
        }],
    };

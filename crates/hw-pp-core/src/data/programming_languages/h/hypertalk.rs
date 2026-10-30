//! Hello-world snippet for `ProgrammingLanguageName::Hypertalk`.
//!
//! Imported from leachim6/hello-world (`h/HyperTalk.ht` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/HyperTalk.ht>
//! - <https://www.wikidata.org/wiki/Q58329> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{EventDriven, Procedural};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Historical, Scripting},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Hypertalk => "HyperTalk",
    versions: [Hypertalk => "HyperTalk"],
    extensions: [Ht => "ht"],
}

const HYPERTALK: &str = concat!(r#"put ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hypertalk,
        extensions: &[FileExtension::Ht],
        categories: &[Scripting, Historical],
        families: &[],
        paradigms: &[Procedural, EventDriven],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hypertalk,
            syntax: HYPERTALK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HYPERTALK,
                    [
                        (TokenKind::Keyword, r"put"),
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

//! Hello-world snippet for `ProgrammingLanguageName::Jess`.
//!
//! Imported from leachim6/hello-world (`j/Jess.cpl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/Jess.cpl>
//! - <https://www.wikidata.org/wiki/Q2259392> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Declarative, Logic};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Jess => "Jess",
    versions: [Jess => "Jess"],
    extensions: [Cpl => "cpl"],
}

const JESS: &str = concat!(r#"(printout t ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jess,
        extensions: &[FileExtension::Cpl],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Declarative, Logic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jess,
            syntax: JESS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JESS,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"printout"),
                        (TokenKind::Plain, r" t "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };

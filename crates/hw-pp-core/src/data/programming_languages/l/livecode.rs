//! Hello-world snippet for `ProgrammingLanguageName::Livecode`.
//!
//! Imported from leachim6/hello-world (`l/LiveCode` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LiveCode>
//! - <https://www.wikidata.org/wiki/Q3068282> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Livecode => "LiveCode",
    versions: [Livecode => "LiveCode"],
    extensions: [],
}

const LIVECODE: &str = concat!(r#"answer ""#, output_for_hello_world!(), r#"" "#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Livecode,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Livecode,
            syntax: LIVECODE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LIVECODE,
                    [
                        (TokenKind::Keyword, r"answer"),
                        (TokenKind::Plain, r" "),
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

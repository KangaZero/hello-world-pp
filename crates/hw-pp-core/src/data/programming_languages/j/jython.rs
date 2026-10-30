//! Hello-world snippet for `ProgrammingLanguageName::Jython`.
//!
//! Imported from leachim6/hello-world (`j/Jython.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/Jython.py>
//! - <https://www.wikidata.org/wiki/Q1053999> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Python, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Jython => "Jython",
    versions: [Jython => "Jython"],
    extensions: [],
}

const JYTHON: &str = concat!(r#"print ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jython,
        extensions: &[],
        categories: &[Scripting],
        families: &[Python],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jython,
            syntax: JYTHON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JYTHON,
                    [
                        (TokenKind::Keyword, r"print"),
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

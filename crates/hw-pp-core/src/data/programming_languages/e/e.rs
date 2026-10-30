//! Hello-world snippet for `ProgrammingLanguageName::E`.
//!
//! Imported from leachim6/hello-world (`e/E.e` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/E.e>
//! - <https://www.wikidata.org/wiki/Q2658244> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: E => "E",
    versions: [E => "E"],
    extensions: [E => "e"],
}

const E: &str = concat!(r#"println(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::E,
        extensions: &[FileExtension::E],
        categories: &[],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::E,
            syntax: E,
            tokenized_syntax: &const {
                checked_token_lengths(
                    E,
                    [(
                        TokenKind::Plain,
                        concat!(r#"println(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };

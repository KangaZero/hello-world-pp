//! Hello-world snippet for `ProgrammingLanguageName::Maude`.
//!
//! Imported from leachim6/hello-world (PR #1770 `m/Maude.maude`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1770>
//! - <https://www.wikidata.org/wiki/Q3558930> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Logic, Metaprogramming, Reflective};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Maude => "Maude",
    versions: [Maude => "Maude"],
    extensions: [Maude => "maude"],
}

const MAUDE: &str = concat!(r#"red ""#, output_for_hello_world!(), r#"" ."#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Maude,
        extensions: &[FileExtension::Maude],
        categories: &[],
        families: &[],
        paradigms: &[Logic, Reflective, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Maude,
            syntax: MAUDE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MAUDE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"red ""#, output_for_hello_world!(), r#"" ."#),
                    )],
                )
            },
        }],
    };

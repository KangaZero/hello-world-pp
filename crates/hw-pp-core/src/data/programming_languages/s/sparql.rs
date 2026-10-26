//! Hello-world snippet for `ProgrammingLanguageName::Sparql`.
//!
//! Imported from leachim6/hello-world (`s/SparQL.sparql` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SparQL.sparql>
//! - <https://www.wikidata.org/wiki/Q54871> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Declarative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Query,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sparql => "SparQL",
    versions: [Sparql => "SparQL"],
    extensions: [Sparql => "sparql"],
}

const SPARQL: &str = concat!(
    r#"SELECT ?h WHERE { 
  VALUES ?h { ""#,
    output_for_hello_world!(),
    r#"" } 
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sparql,
        extensions: &[FileExtension::Sparql],
        categories: &[Query],
        families: &[],
        paradigms: &[Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sparql,
            syntax: SPARQL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SPARQL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"SELECT ?h WHERE { 
  VALUES ?h { ""#,
                            output_for_hello_world!(),
                            r#"" } 
}"#
                        ),
                    )],
                )
            },
        }],
    };

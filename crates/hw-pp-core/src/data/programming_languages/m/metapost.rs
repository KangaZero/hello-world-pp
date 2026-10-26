//! Hello-world snippet for `ProgrammingLanguageName::Metapost`.
//!
//! Imported from the Hello World Collection (`files/MetaPost.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MetaPost.htm>
//! - <http://helloworldcollection.de/#MetaPost>
//! - <https://www.wikidata.org/wiki/Q1265836> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Declarative, Imperative};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Metapost => "MetaPost",
    versions: [Metapost => "MetaPost"],
    extensions: [],
}

const METAPOST: &str = concat!(
    r"% ",
    output_for_hello_world!(),
    r#" in MetaPost
beginfig(0);
  label(""#,
    output_for_hello_world!(),
    r#"", (10,10));
endfig;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Metapost,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Metapost,
            syntax: METAPOST,
            tokenized_syntax: &const {
                checked_token_lengths(
                    METAPOST,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"% ",
                            output_for_hello_world!(),
                            r#" in MetaPost
beginfig(0);
  label(""#,
                            output_for_hello_world!(),
                            r#"", (10,10));
endfig;"#
                        ),
                    )],
                )
            },
        }],
    };

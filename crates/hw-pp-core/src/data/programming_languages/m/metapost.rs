//! Hello-world snippet for `ProgrammingLanguageName::Metapost`.
//!
//! Imported from the Hello World Collection (`files/MetaPost.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MetaPost.htm>
//! - <http://helloworldcollection.de/#MetaPost>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Metapost => "MetaPost",
    versions: [Metapost => "MetaPost"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Metapost,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Metapost,
            syntax: concat!(
                r"% ",
                output_for_hello_world!(),
                r#" in MetaPost
beginfig(0);
  label(""#,
                output_for_hello_world!(),
                r#"", (10,10));
endfig;"#
            ),
            tokenized_syntax: &[(
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
        }],
    };

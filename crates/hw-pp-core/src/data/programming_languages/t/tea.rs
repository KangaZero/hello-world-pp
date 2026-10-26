//! Hello-world snippet for `ProgrammingLanguageName::Tea`.
//!
//! Imported from leachim6/hello-world (`t/Tea.tea` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Tea.tea>
//! - <https://www.wikidata.org/wiki/Q3516674> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, ObjectOriented};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Tea => "Tea",
    versions: [Tea => "Tea"],
    extensions: [],
}

const TEA: &str = concat!(r#"echo ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tea,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tea,
            syntax: TEA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TEA,
                    [(
                        TokenKind::Plain,
                        concat!(r#"echo ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };

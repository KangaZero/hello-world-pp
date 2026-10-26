//! Hello-world snippet for `ProgrammingLanguageName::Xquery`.
//!
//! Imported from leachim6/hello-world (`x/XQuery.xq` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XQuery.xq>
//! - <https://www.wikidata.org/wiki/Q850661> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Declarative, Functional};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Query,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Xquery => "XQuery",
    versions: [Xquery => "XQuery"],
    extensions: [Xq => "xq"],
}

const XQUERY: &str = concat!(
    r#"let $hello := ""#,
    output_for_hello_world!(),
    r#""
return $hello"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xquery,
        extensions: &[FileExtension::Xq],
        categories: &[Query],
        families: &[],
        paradigms: &[Functional, Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xquery,
            syntax: XQUERY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XQUERY,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"let $hello := ""#,
                            output_for_hello_world!(),
                            r#""
return $hello"#
                        ),
                    )],
                )
            },
        }],
    };

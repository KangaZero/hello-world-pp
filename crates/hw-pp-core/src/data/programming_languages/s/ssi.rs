//! Hello-world snippet for `ProgrammingLanguageName::Ssi`.
//!
//! Imported from the Hello World Collection (`files/SSI.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SSI.htm>
//! - <http://helloworldcollection.de/#SSI>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Ssi => "SSI",
    versions: [Ssi => "SSI"],
    extensions: [],
}

const SSI: &str = concat!(
    r"<html><body><p>
<!-- ",
    output_for_hello_world!(),
    r#" as Server Side Includes, embedded in HTML. -->
<!--#set var="hello" value=""#,
    output_for_hello_world!(),
    r#"" -->
<!--#echo var="hello" -->
</p></body></html>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ssi,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ssi,
            syntax: SSI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SSI,
                    [
                        (TokenKind::Keyword, r"<html><body><p>"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            concat!(
                                r"<!-- ",
                                output_for_hello_world!(),
                                r" as Server Side Includes, embedded in HTML. -->"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            concat!(
                                r#"<!--#set var="hello" value=""#,
                                output_for_hello_world!(),
                                r#"" -->"#
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r#"<!--#echo var="hello" -->"#),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"</p></body></html>"),
                    ],
                )
            },
        }],
    };

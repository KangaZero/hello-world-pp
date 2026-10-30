//! Hello-world snippet for `ProgrammingLanguageName::JavaServerPages`.
//!
//! Imported from the Hello World Collection (`files/Java&nbsp;Server&nbsp;Pages.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Java%26nbsp%3BServer%26nbsp%3BPages.txt>
//! - <http://helloworldcollection.de/#Java%C2%A0Server%C2%A0Pages>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: JavaServerPages => "Java Server Pages",
    versions: [JavaServerPages => "Java Server Pages"],
    extensions: [],
}

const JAVA_SERVER_PAGES: &str = concat!(
    r"<!-- ",
    output_for_hello_world!(),
    r#" for Java Server Pages -->

<%@ page language='java' %>
<%=""#,
    output_for_hello_world!(),
    r#"" %>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::JavaServerPages,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::JavaServerPages,
            syntax: JAVA_SERVER_PAGES,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JAVA_SERVER_PAGES,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"<!-- ",
                                output_for_hello_world!(),
                                r" for Java Server Pages -->"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"<"),
                        (TokenKind::Plain, r"%@ page language="),
                        (TokenKind::String, r"'java'"),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Keyword, r">"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"<"),
                        (TokenKind::Plain, r"%="),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Plain, r" %"),
                        (TokenKind::Keyword, r">"),
                    ],
                )
            },
        }],
    };

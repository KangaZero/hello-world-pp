//! Hello-world snippet for `ProgrammingLanguageName::Birl`.
//!
//! Imported from leachim6/hello-world (`b/BIRL.c` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Birl => "BIRL",
    versions: [Birl => "BIRL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Birl,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Birl,
            syntax: concat!(
                r#"HORA DO SHOW // main code block
    CE QUER VER ESSA PORRA? (""#,
                output_for_hello_world!(),
                r#"\n"); // print ""#,
                output_for_hello_world!(),
                r#"""
    BORA CUMPADE 0; // end function
BIRL"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"HORA DO SHOW // main code block
    CE QUER VER ESSA PORRA? (""#,
                    output_for_hello_world!(),
                    r#"\n"); // print ""#,
                    output_for_hello_world!(),
                    r#"""
    BORA CUMPADE 0; // end function
BIRL"#
                ),
            )],
        }],
    };

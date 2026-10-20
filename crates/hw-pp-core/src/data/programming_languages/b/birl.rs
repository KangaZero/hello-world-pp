//! Hello-world snippet for `ProgrammingLanguageName::Birl`.
//!
//! Imported from leachim6/hello-world (`b/BIRL.c` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BIRL.c>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Esolang],
        families: &[],
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

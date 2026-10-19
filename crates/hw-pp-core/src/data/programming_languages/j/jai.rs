//! Hello-world snippet for `ProgrammingLanguageName::Jai`.
//!
//! Imported from leachim6/hello-world (`j/Jai.jai` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Jai => "Jai",
    versions: [Jai => "Jai"],
    extensions: [Jai => "jai"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jai,
        extensions: &[FileExtension::Jai],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jai,
            syntax: concat!(
                r#"#import "Print"

main::(){

	print(""#,
                output_for_hello_world!(),
                r#"");

}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#import "Print"

main::(){

	print(""#,
                    output_for_hello_world!(),
                    r#"");

}"#
                ),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Cobra`.
//!
//! Imported from leachim6/hello-world (`c/Cobra.cobra` at `a152253`),
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
    language: Cobra => "Cobra",
    versions: [Cobra => "Cobra"],
    extensions: [Cobra => "cobra"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cobra,
        extensions: &[FileExtension::Cobra],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cobra,
            syntax: concat!(
                r"class Hello

    def main
        print '",
                output_for_hello_world!(),
                r"'"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"class Hello

    def main
        print '",
                    output_for_hello_world!(),
                    r"'"
                ),
            )],
        }],
    };

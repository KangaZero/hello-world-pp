//! Hello-world snippet for `ProgrammingLanguageName::Ceylon`.
//!
//! Imported from leachim6/hello-world (`c/Ceylon.ceylon` at `a152253`),
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
    language: Ceylon => "Ceylon",
    versions: [Ceylon => "Ceylon"],
    extensions: [Ceylon => "ceylon"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ceylon,
        extensions: &[FileExtension::Ceylon],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ceylon,
            syntax: concat!(
                r#"shared void hello() {
    print(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"shared void hello() {
    print(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };

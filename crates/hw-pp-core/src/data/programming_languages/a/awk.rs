//! Hello-world snippet for `ProgrammingLanguageName::Awk`.
//!
//! Imported from leachim6/hello-world (`a/AWK.awk` at `a152253`),
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
    language: Awk => "AWK",
    versions: [Awk => "AWK"],
    extensions: [Awk => "awk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Awk,
        extensions: &[FileExtension::Awk],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Awk,
            syntax: concat!(
                r#"# awk -f awk.awk
BEGIN { print ""#,
                output_for_hello_world!(),
                r#"" }"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"# awk -f awk.awk
BEGIN { print ""#,
                    output_for_hello_world!(),
                    r#"" }"#
                ),
            )],
        }],
    };

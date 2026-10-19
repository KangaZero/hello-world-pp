//! Hello-world snippet for `ProgrammingLanguageName::Millfork`.
//!
//! Imported from leachim6/hello-world (`m/Millfork.mfk` at `a152253`),
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
    language: Millfork => "Millfork",
    versions: [Millfork => "Millfork"],
    extensions: [Mfk => "mfk"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Millfork,
        extensions: &[FileExtension::Mfk],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Millfork,
            syntax: concat!(
                r#"import stdio

void main() {
    putstrz(""#,
                output_for_hello_world!(),
                r#""z)
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"import stdio

void main() {
    putstrz(""#,
                    output_for_hello_world!(),
                    r#""z)
}"#
                ),
            )],
        }],
    };

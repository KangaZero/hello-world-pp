//! Hello-world snippet for `ProgrammingLanguageName::Pike`.
//!
//! Imported from leachim6/hello-world (`p/Pike.pike` at `a152253`),
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
    language: Pike => "Pike",
    versions: [Pike => "Pike"],
    extensions: [Pike => "pike"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pike,
        extensions: &[FileExtension::Pike],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pike,
            syntax: concat!(
                r#"int main() {
    write(""#,
                output_for_hello_world!(),
                r#"\n");
    return 0;
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"int main() {
    write(""#,
                    output_for_hello_world!(),
                    r#"\n");
    return 0;
}"#
                ),
            )],
        }],
    };

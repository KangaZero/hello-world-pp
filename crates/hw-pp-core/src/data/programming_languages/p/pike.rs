//! Hello-world snippet for `ProgrammingLanguageName::Pike`.
//!
//! Imported from leachim6/hello-world (`p/Pike.pike` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pike.pike>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Scripting],
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

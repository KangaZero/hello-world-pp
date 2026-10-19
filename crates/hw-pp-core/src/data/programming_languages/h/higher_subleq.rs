//! Hello-world snippet for `ProgrammingLanguageName::HigherSubleq`.
//!
//! Imported from leachim6/hello-world (`h/Higher Subleq.higher_subleq` at `a152253`),
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
    language: HigherSubleq => "Higher Subleq",
    versions: [HigherSubleq => "Higher Subleq"],
    extensions: [HigherSubleq => "higher_subleq"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HigherSubleq,
        extensions: &[FileExtension::HigherSubleq],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HigherSubleq,
            syntax: concat!(
                r#"int printf();
int main()
{
  printf(""#,
                output_for_hello_world!(),
                r#"\n");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"int printf();
int main()
{
  printf(""#,
                    output_for_hello_world!(),
                    r#"\n");
}"#
                ),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::HigherSubleq`.
//!
//! Imported from leachim6/hello-world (`h/Higher Subleq.higher_subleq` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Higher%20Subleq.higher_subleq>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: HigherSubleq => "Higher Subleq",
    versions: [HigherSubleq => "Higher Subleq"],
    extensions: [HigherSubleq => "higher_subleq"],
}

const HIGHER_SUBLEQ: &str = concat!(
    r#"int printf();
int main()
{
  printf(""#,
    output_for_hello_world!(),
    r#"\n");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HigherSubleq,
        extensions: &[FileExtension::HigherSubleq],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HigherSubleq,
            syntax: HIGHER_SUBLEQ,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HIGHER_SUBLEQ,
                    [(
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
                )
            },
        }],
    };

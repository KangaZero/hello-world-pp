//! Hello-world snippet for `ProgrammingLanguageName::Promela`.
//!
//! Imported from leachim6/hello-world (`p/Promela.pml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Promela.pml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Promela => "Promela",
    versions: [Promela => "Promela"],
    extensions: [Pml => "pml"],
}

const PROMELA: &str = concat!(
    r#"active proctype main(){
  printf(""#,
    output_for_hello_world!(),
    r#"")
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Promela,
        extensions: &[FileExtension::Pml],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Promela,
            syntax: PROMELA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PROMELA,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"active proctype main(){
  printf(""#,
                            output_for_hello_world!(),
                            r#"")
}"#
                        ),
                    )],
                )
            },
        }],
    };

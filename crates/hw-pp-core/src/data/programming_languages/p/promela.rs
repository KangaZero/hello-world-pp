//! Hello-world snippet for `ProgrammingLanguageName::Promela`.
//!
//! Imported from leachim6/hello-world (`p/Promela.pml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Promela.pml>
//! - <https://en.wikipedia.org/wiki/Promela> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Concurrent;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

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
        paradigms: &[Concurrent],
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

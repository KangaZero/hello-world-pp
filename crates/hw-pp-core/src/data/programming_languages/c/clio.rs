//! Hello-world snippet for `ProgrammingLanguageName::Clio`.
//!
//! Imported from leachim6/hello-world (`c/Clio.clio` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Clio.clio>
//! - <https://github.com/clio-lang/clio> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Clio => "Clio",
    versions: [Clio => "Clio"],
    extensions: [Clio => "clio"],
}

const CLIO: &str = concat!(
    r#"export fn main argv:
  ""#,
    output_for_hello_world!(),
    r#"" -> console.log"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clio,
        extensions: &[FileExtension::Clio],
        categories: &[],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clio,
            syntax: CLIO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CLIO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"export fn main argv:
  ""#,
                            output_for_hello_world!(),
                            r#"" -> console.log"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Cone`.
//!
//! Imported from leachim6/hello-world (`c/Cone.cone` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cone.cone>
//! - <https://cone.jondgoodwin.com/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Concurrent;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Cone => "Cone",
    versions: [Cone => "Cone"],
    extensions: [Cone => "cone"],
}

const CONE: &str = concat!(
    r#"import stdio::*

fn main():
  print <- ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cone,
        extensions: &[FileExtension::Cone],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cone,
            syntax: CONE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CONE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"import stdio::*

fn main():
  print <- ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };

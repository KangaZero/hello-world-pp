//! Hello-world snippet for `ProgrammingLanguageName::Eo`.
//!
//! Imported from leachim6/hello-world (`e/EO.eo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/EO.eo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Eo => "EO",
    versions: [Eo => "EO"],
    extensions: [Eo => "eo"],
}

const EO: &str = concat!(
    r#"+alias org.eolang.io.stdout

[] > app
  stdout > @
    ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Eo,
        extensions: &[FileExtension::Eo],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Eo,
            syntax: EO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"+alias org.eolang.io.stdout

[] > app
  stdout > @
    ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };

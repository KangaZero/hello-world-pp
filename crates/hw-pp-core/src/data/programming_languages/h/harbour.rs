//! Hello-world snippet for `ProgrammingLanguageName::Harbour`.
//!
//! Imported from leachim6/hello-world (`h/Harbour.prg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Harbour.prg>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::XBase, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Harbour => "Harbour",
    versions: [Harbour => "Harbour"],
    extensions: [],
}

const HARBOUR: &str = concat!(
    r#"PROCEDURE Main()

   ? ""#,
    output_for_hello_world!(),
    r#""

   RETURN"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Harbour,
        extensions: &[],
        categories: &[Compiled],
        families: &[XBase],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Harbour,
            syntax: HARBOUR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HARBOUR,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"PROCEDURE Main()

   ? ""#,
                            output_for_hello_world!(),
                            r#""

   RETURN"#
                        ),
                    )],
                )
            },
        }],
    };

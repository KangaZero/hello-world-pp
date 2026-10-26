//! Hello-world snippet for `ProgrammingLanguageName::N4gl`.
//!
//! Imported from leachim6/hello-world (`#/4gl.4gl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/4gl.4gl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Historical,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N4gl => "4gl",
    versions: [N4gl => "4gl"],
    extensions: [E4gl => "4gl"],
}

const N4GL: &str = concat!(
    r#"main
    display ""#,
    output_for_hello_world!(),
    r#""
end main"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N4gl,
        extensions: &[FileExtension::E4gl],
        categories: &[Historical],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N4gl,
            syntax: N4GL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N4GL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"main
    display ""#,
                            output_for_hello_world!(),
                            r#""
end main"#
                        ),
                    )],
                )
            },
        }],
    };

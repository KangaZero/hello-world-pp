//! Hello-world snippet for `ProgrammingLanguageName::Scss`.
//!
//! Imported from leachim6/hello-world (`s/SCSS.scss` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SCSS.scss>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Scss => "SCSS",
    versions: [Scss => "SCSS"],
    extensions: [Scss => "scss"],
}

const SCSS: &str = concat!(
    r#"body::before {
    content: ""#,
    output_for_hello_world!(),
    r#"";
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Scss,
        extensions: &[FileExtension::Scss],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Scss,
            syntax: SCSS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SCSS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"body::before {
    content: ""#,
                            output_for_hello_world!(),
                            r#"";
}"#
                        ),
                    )],
                )
            },
        }],
    };

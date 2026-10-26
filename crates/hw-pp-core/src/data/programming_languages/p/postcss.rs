//! Hello-world snippet for `ProgrammingLanguageName::Postcss`.
//!
//! Imported from leachim6/hello-world (`p/PostCSS.pcss` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PostCSS.pcss>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Postcss => "PostCSS",
    versions: [Postcss => "PostCSS"],
    extensions: [Pcss => "pcss"],
}

const POSTCSS: &str = concat!(
    r#"body::before {
    content: ""#,
    output_for_hello_world!(),
    r#"";
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Postcss,
        extensions: &[FileExtension::Pcss],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Postcss,
            syntax: POSTCSS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POSTCSS,
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

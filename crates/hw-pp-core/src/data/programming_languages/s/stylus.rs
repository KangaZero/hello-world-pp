//! Hello-world snippet for `ProgrammingLanguageName::Stylus`.
//!
//! Imported from leachim6/hello-world (`s/Stylus.styl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Stylus.styl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Stylus => "Stylus",
    versions: [Stylus => "Stylus"],
    extensions: [Styl => "styl"],
}

const STYLUS: &str = concat!(
    r#"body::before
	content: ""#,
    output_for_hello_world!(),
    r#"" "#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Stylus,
        extensions: &[FileExtension::Styl],
        categories: &[Markup],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Stylus,
            syntax: STYLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    STYLUS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"body::before
	content: ""#,
                            output_for_hello_world!(),
                            r#"" "#
                        ),
                    )],
                )
            },
        }],
    };

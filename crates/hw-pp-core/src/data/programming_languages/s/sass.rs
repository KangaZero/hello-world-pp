//! Hello-world snippet for `ProgrammingLanguageName::Sass`.
//!
//! Imported from leachim6/hello-world (`s/Sass.sass` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Sass.sass>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Sass => "Sass",
    versions: [Sass => "Sass"],
    extensions: [Sass => "sass"],
}

const SASS: &str = concat!(
    r#"body::before
	content: ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sass,
        extensions: &[FileExtension::Sass],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sass,
            syntax: SASS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SASS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"body::before
	content: ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Flascript`.
//!
//! Imported from leachim6/hello-world (`f/FlaScript.fls` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/FlaScript.fls>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Flascript => "FlaScript",
    versions: [Flascript => "FlaScript"],
    extensions: [Fls => "fls"],
}

const FLASCRIPT: &str = concat!(
    r#"main() -> main {
	print(string) -> ""#,
    output_for_hello_world!(),
    r#""
	print(newline)
} main;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flascript,
        extensions: &[FileExtension::Fls],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flascript,
            syntax: FLASCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FLASCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"main() -> main {
	print(string) -> ""#,
                            output_for_hello_world!(),
                            r#""
	print(newline)
} main;"#
                        ),
                    )],
                )
            },
        }],
    };

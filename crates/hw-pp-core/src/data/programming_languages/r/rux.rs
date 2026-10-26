//! Hello-world snippet for `ProgrammingLanguageName::Rux`.
//!
//! Imported from leachim6/hello-world (PR #1826 `r/Rux.rux`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1826>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rux => "Rux",
    versions: [Rux => "Rux"],
    extensions: [Rux => "rux"],
}

const RUX: &str = concat!(
    r#"import Std::Io::Print;

func Main() -> int {
	Print(""#,
    output_for_hello_world!(),
    r#"");
	return 0;
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rux,
        extensions: &[FileExtension::Rux],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rux,
            syntax: RUX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RUX,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"import Std::Io::Print;

func Main() -> int {
	Print(""#,
                            output_for_hello_world!(),
                            r#"");
	return 0;
}"#
                        ),
                    )],
                )
            },
        }],
    };

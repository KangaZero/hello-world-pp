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
    programming_languages_token::TokenKind,
};

language_names! {
    language: Rux => "Rux",
    versions: [Rux => "Rux"],
    extensions: [Rux => "rux"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rux,
        extensions: &[FileExtension::Rux],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rux,
            syntax: concat!(
                r#"import Std::Io::Print;

func Main() -> int {
	Print(""#,
                output_for_hello_world!(),
                r#"");
	return 0;
}"#
            ),
            tokenized_syntax: &[(
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
        }],
    };

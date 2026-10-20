//! Hello-world snippet for `ProgrammingLanguageName::Gaml`.
//!
//! Imported from leachim6/hello-world (`g/GAML.gaml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GAML.gaml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Gaml => "GAML",
    versions: [Gaml => "GAML"],
    extensions: [Gaml => "gaml"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gaml,
        extensions: &[FileExtension::Gaml],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gaml,
            syntax: concat!(
                r#"model GAML

global {
	init {
		write(""#,
                output_for_hello_world!(),
                r#"");
	}
}

experiment GAML {}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"model GAML

global {
	init {
		write(""#,
                    output_for_hello_world!(),
                    r#"");
	}
}

experiment GAML {}"#
                ),
            )],
        }],
    };

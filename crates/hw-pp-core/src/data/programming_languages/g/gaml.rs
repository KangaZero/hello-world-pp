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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Gaml => "GAML",
    versions: [Gaml => "GAML"],
    extensions: [Gaml => "gaml"],
}

const GAML: &str = concat!(
    r#"model GAML

global {
	init {
		write(""#,
    output_for_hello_world!(),
    r#"");
	}
}

experiment GAML {}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gaml,
        extensions: &[FileExtension::Gaml],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gaml,
            syntax: GAML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GAML,
                    [(
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
                )
            },
        }],
    };

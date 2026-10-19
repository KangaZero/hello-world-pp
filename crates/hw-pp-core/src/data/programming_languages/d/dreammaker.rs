//! Hello-world snippet for `ProgrammingLanguageName::Dreammaker`.
//!
//! Imported from leachim6/hello-world (`d/DreamMaker.dm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dreammaker => "DreamMaker",
    versions: [Dreammaker => "DreamMaker"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dreammaker,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dreammaker,
            syntax: concat!(
                r#"mob
	Login()
		world << ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"mob
	Login()
		world << ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };

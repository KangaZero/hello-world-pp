//! Hello-world snippet for `ProgrammingLanguageName::Emerald`.
//!
//! Imported from leachim6/hello-world (`e/Emerald.m` at `a152253`),
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
    language: Emerald => "Emerald",
    versions: [Emerald => "Emerald"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emerald,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emerald,
            syntax: concat!(
                r#"const main <- object mainProgram
    initially
		stdout.putString[""#,
                output_for_hello_world!(),
                r#"\n"]
	end initially
end mainProgram"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"const main <- object mainProgram
    initially
		stdout.putString[""#,
                    output_for_hello_world!(),
                    r#"\n"]
	end initially
end mainProgram"#
                ),
            )],
        }],
    };

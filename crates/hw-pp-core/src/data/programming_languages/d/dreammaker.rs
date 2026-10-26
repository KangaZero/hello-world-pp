//! Hello-world snippet for `ProgrammingLanguageName::Dreammaker`.
//!
//! Imported from leachim6/hello-world (`d/DreamMaker.dm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DreamMaker.dm>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Dreammaker => "DreamMaker",
    versions: [Dreammaker => "DreamMaker"],
    extensions: [],
}

const DREAMMAKER: &str = concat!(
    r#"mob
	Login()
		world << ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dreammaker,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dreammaker,
            syntax: DREAMMAKER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DREAMMAKER,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"mob
	Login()
		world << ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };

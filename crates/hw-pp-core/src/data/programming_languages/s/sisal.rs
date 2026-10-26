//! Hello-world snippet for `ProgrammingLanguageName::Sisal`.
//!
//! Imported from leachim6/hello-world (PR #1490 `s/Sisal.sis`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1490>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sisal => "Sisal",
    versions: [Sisal => "Sisal"],
    extensions: [Sis => "sis"],
}

const SISAL: &str = concat!(
    r#"define main

type string = array[character];

function main(returns string)
  ""#,
    output_for_hello_world!(),
    r#""
end function"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sisal,
        extensions: &[FileExtension::Sis],
        categories: &[Compiled, Historical],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sisal,
            syntax: SISAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SISAL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"define main

type string = array[character];

function main(returns string)
  ""#,
                            output_for_hello_world!(),
                            r#""
end function"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Sisal`.
//!
//! Imported from leachim6/hello-world (PR #1490 `s/Sisal.sis`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sisal => "Sisal",
    versions: [Sisal => "Sisal"],
    extensions: [Sis => "sis"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sisal,
        extensions: &[FileExtension::Sis],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sisal,
            syntax: concat!(
                r#"define main

type string = array[character];

function main(returns string)
  ""#,
                output_for_hello_world!(),
                r#""
end function"#
            ),
            tokenized_syntax: &[(
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
        }],
    };

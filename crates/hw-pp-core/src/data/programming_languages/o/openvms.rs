//! Hello-world snippet for `ProgrammingLanguageName::Openvms`.
//!
//! Imported from the Hello World Collection (`files/OpenVMS.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/OpenVMS.txt>
//! - <http://helloworldcollection.de/#OpenVMS>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Openvms => "OpenVMS",
    versions: [Openvms => "OpenVMS"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Openvms,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Openvms,
            syntax: concat!(
                r"$! ",
                output_for_hello_world!(),
                r#" in OpenVMS DCL

$ write sys$output ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"$! ",
                    output_for_hello_world!(),
                    r#" in OpenVMS DCL

$ write sys$output ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Ti8x`.
//!
//! Imported from the Hello World Collection (`files/TI-8x` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/TI-8x>
//! - <http://helloworldcollection.de/#TI-8x>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ti8x => "TI-8x",
    versions: [Ti8x => "TI-8x"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ti8x,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ti8x,
            syntax: concat!(
                output_for_hello_world!(),
                r#" for TI 8x/9x basic (tested on a TI-83)

:ClrHome
:Disp ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r#" for TI 8x/9x basic (tested on a TI-83)

:ClrHome
:Disp ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };

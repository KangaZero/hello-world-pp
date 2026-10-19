//! Hello-world snippet for `ProgrammingLanguageName::Setl2`.
//!
//! Imported from the Hello World Collection (`files/Setl2` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Setl2>
//! - <http://helloworldcollection.de/#Setl2>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Setl2 => "Setl2",
    versions: [Setl2 => "Setl2"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Setl2,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Setl2,
            syntax: concat!(
                r"-- ",
                output_for_hello_world!(),
                r#" in Setl2

procedure Hello();
   print ""#,
                output_for_hello_world!(),
                r#"";
end Hello;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"-- ",
                    output_for_hello_world!(),
                    r#" in Setl2

procedure Hello();
   print ""#,
                    output_for_hello_world!(),
                    r#"";
end Hello;"#
                ),
            )],
        }],
    };

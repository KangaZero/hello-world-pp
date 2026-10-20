//! Hello-world snippet for `ProgrammingLanguageName::RationalRose`.
//!
//! Imported from the Hello World Collection (`files/Rational&nbsp;Rose.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Rational%26nbsp%3BRose.htm>
//! - <http://helloworldcollection.de/#Rational%C2%A0Rose>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: RationalRose => "Rational Rose",
    versions: [RationalRose => "Rational Rose"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RationalRose,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RationalRose,
            syntax: concat!(
                r"' ",
                output_for_hello_world!(),
                r#" in Rational Rose scripting language
Sub Main
    RoseApp.WriteErrorLog ""#,
                output_for_hello_world!(),
                r#""	
End Sub"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"' ",
                    output_for_hello_world!(),
                    r#" in Rational Rose scripting language
Sub Main
    RoseApp.WriteErrorLog ""#,
                    output_for_hello_world!(),
                    r#""	
End Sub"#
                ),
            )],
        }],
    };

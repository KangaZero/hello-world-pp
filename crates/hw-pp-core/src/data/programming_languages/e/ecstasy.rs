//! Hello-world snippet for `ProgrammingLanguageName::Ecstasy`.
//!
//! Imported from the Hello World Collection (`files/Ecstasy.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Ecstasy.htm>
//! - <http://helloworldcollection.de/#Ecstasy>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ecstasy => "Ecstasy",
    versions: [Ecstasy => "Ecstasy"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ecstasy,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ecstasy,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in Ecstasy

module HelloWorld
    {
    void run()
        {
        @Inject Console console;
        console.println(""#,
                output_for_hello_world!(),
                r#"");
        }
    }"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in Ecstasy

module HelloWorld
    {
    void run()
        {
        @Inject Console console;
        console.println(""#,
                    output_for_hello_world!(),
                    r#"");
        }
    }"#
                ),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Vsl`.
//!
//! Imported from the Hello World Collection (`files/VSL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/VSL.txt>
//! - <http://helloworldcollection.de/#VSL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Vsl => "VSL",
    versions: [Vsl => "VSL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vsl,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vsl,
            syntax: concat!(
                r#"/* ""#,
                output_for_hello_world!(),
                r#"" in VSL (the script language of Virtools),
to be used in a "Run VSL" building block */
 
void main()
{
    bc.OutputToScreen(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"/* ""#,
                    output_for_hello_world!(),
                    r#"" in VSL (the script language of Virtools),
to be used in a "Run VSL" building block */
 
void main()
{
    bc.OutputToScreen(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };

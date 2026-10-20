//! Hello-world snippet for `ProgrammingLanguageName::Mex`.
//!
//! Imported from the Hello World Collection (`files/MEX.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MEX.htm>
//! - <http://helloworldcollection.de/#MEX>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mex => "MEX",
    versions: [Mex => "MEX"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mex,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mex,
            syntax: concat!(
                r#"// ""#,
                output_for_hello_world!(),
                r#"" in MEX for Maximus BBS

void main()
{
  print(COLOR_WHITE,""#,
                output_for_hello_world!(),
                r#"\n");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"// ""#,
                    output_for_hello_world!(),
                    r#"" in MEX for Maximus BBS

void main()
{
  print(COLOR_WHITE,""#,
                    output_for_hello_world!(),
                    r#"\n");
}"#
                ),
            )],
        }],
    };

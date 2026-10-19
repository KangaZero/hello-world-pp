//! Hello-world snippet for `ProgrammingLanguageName::Msil`.
//!
//! Imported from the Hello World Collection (`files/MSIL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MSIL.txt>
//! - <http://helloworldcollection.de/#MSIL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Msil => "MSIL",
    versions: [Msil => "MSIL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Msil,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Msil,
            syntax: concat!(
                r"//",
                output_for_hello_world!(),
                r#" in MSIL (.NET assembler)

.assembly helloworld {}
.class helloworld
{
 .method static void Main() cil managed
 {
  .entrypoint
  ldstr ""#,
                output_for_hello_world!(),
                r#""
  call void [mscorlib]System.Console::WriteLine(string)
  ret
 }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"//",
                    output_for_hello_world!(),
                    r#" in MSIL (.NET assembler)

.assembly helloworld {}
.class helloworld
{
 .method static void Main() cil managed
 {
  .entrypoint
  ldstr ""#,
                    output_for_hello_world!(),
                    r#""
  call void [mscorlib]System.Console::WriteLine(string)
  ret
 }
}"#
                ),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::CSlashAl`.
//!
//! Imported from the Hello World Collection (`files/C&#47;AL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%26%2347%3BAL.txt>
//! - <http://helloworldcollection.de/#C%2FAL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Pascal,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: CSlashAl => "C/AL",
    versions: [CSlashAl => "C/AL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CSlashAl,
        extensions: &[],
        categories: &[],
        families: &[Pascal],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CSlashAl,
            syntax: concat!(
                r"OBJECT Codeunit 50000 HelloWorld
{
  PROPERTIES
  {
    OnRun=BEGIN
            //",
                output_for_hello_world!(),
                r" in C/AL (Microsoft Business Solutions-Navision)
            MESSAGE(Txt001);
          END;
  }
  CODE
  {
    VAR
      Txt001@1000000000 : TextConst 'ENU=",
                output_for_hello_world!(),
                r"';
    BEGIN
    END.
  }
}"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"OBJECT Codeunit 50000 HelloWorld
{
  PROPERTIES
  {
    OnRun=BEGIN
            //",
                    output_for_hello_world!(),
                    r" in C/AL (Microsoft Business Solutions-Navision)
            MESSAGE(Txt001);
          END;
  }
  CODE
  {
    VAR
      Txt001@1000000000 : TextConst 'ENU=",
                    output_for_hello_world!(),
                    r"';
    BEGIN
    END.
  }
}"
                ),
            )],
        }],
    };

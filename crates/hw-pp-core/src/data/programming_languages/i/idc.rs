//! Hello-world snippet for `ProgrammingLanguageName::Idc`.
//!
//! Imported from the Hello World Collection (`files/IDC.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/IDC.txt>
//! - <http://helloworldcollection.de/#IDC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Idc => "IDC",
    versions: [Idc => "IDC"],
    extensions: [],
}

const IDC: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in IDC-script language for IDA disaasembler

#include <idc.idc>

static main(void)
{
  Message(""#,
    output_for_hello_world!(),
    r#"");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Idc,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Idc,
            syntax: IDC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    IDC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in IDC-script language for IDA disaasembler

#include <idc.idc>

static main(void)
{
  Message(""#,
                            output_for_hello_world!(),
                            r#"");
}"#
                        ),
                    )],
                )
            },
        }],
    };

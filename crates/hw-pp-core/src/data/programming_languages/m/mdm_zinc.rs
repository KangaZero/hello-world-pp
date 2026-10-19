//! Hello-world snippet for `ProgrammingLanguageName::MdmZinc`.
//!
//! Imported from the Hello World Collection (`files/MDM&nbsp;Zinc.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MDM%26nbsp%3BZinc.htm>
//! - <http://helloworldcollection.de/#MDM%C2%A0Zinc>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: MdmZinc => "MDM Zinc",
    versions: [MdmZinc => "MDM Zinc"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MdmZinc,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MdmZinc,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in MDM Zinc

mdm.Dialogs.prompt(""#,
                output_for_hello_world!(),
                r#"");
mdm.Application.exit();"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in MDM Zinc

mdm.Dialogs.prompt(""#,
                    output_for_hello_world!(),
                    r#"");
mdm.Application.exit();"#
                ),
            )],
        }],
    };

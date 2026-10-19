//! Hello-world snippet for `ProgrammingLanguageName::Installscript`.
//!
//! Imported from the Hello World Collection (`files/InstallScript` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/InstallScript>
//! - <http://helloworldcollection.de/#InstallScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Installscript => "InstallScript",
    versions: [Installscript => "InstallScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Installscript,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Installscript,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in InstallScript
// (Scripting language of InstallShield, a Windows install generator)

program
	MessageBox(""#,
                output_for_hello_world!(),
                r#"",INFORMATION);
endprogram"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in InstallScript
// (Scripting language of InstallShield, a Windows install generator)

program
	MessageBox(""#,
                    output_for_hello_world!(),
                    r#"",INFORMATION);
endprogram"#
                ),
            )],
        }],
    };

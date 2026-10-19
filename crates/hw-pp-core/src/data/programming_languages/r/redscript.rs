//! Hello-world snippet for `ProgrammingLanguageName::Redscript`.
//!
//! Imported from leachim6/hello-world (PR #1790 `r/Redscript.reds`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1790>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Redscript => "Redscript",
    versions: [Redscript => "Redscript"],
    extensions: [Reds => "reds"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Redscript,
        extensions: &[FileExtension::Reds],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Redscript,
            syntax: concat!(
                r#"@wrapMethod(EquipmentSystemPlayerData)
func OnRestored() -> Void {
  wrappedMethod();
  Log(""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"@wrapMethod(EquipmentSystemPlayerData)
func OnRestored() -> Void {
  wrappedMethod();
  Log(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };

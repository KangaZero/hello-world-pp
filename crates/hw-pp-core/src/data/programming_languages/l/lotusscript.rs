//! Hello-world snippet for `ProgrammingLanguageName::Lotusscript`.
//!
//! Imported from leachim6/hello-world (`l/LotusScript.lss` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Lotusscript => "LotusScript",
    versions: [Lotusscript => "LotusScript"],
    extensions: [Lss => "lss"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lotusscript,
        extensions: &[FileExtension::Lss],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lotusscript,
            syntax: concat!(
                r#"Sub Initialize
	Print ""#,
                output_for_hello_world!(),
                r#""
End Sub"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"Sub Initialize
	Print ""#,
                    output_for_hello_world!(),
                    r#""
End Sub"#
                ),
            )],
        }],
    };

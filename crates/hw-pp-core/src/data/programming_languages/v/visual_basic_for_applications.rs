//! Hello-world snippet for `ProgrammingLanguageName::VisualBasicForApplications`.
//!
//! Imported from leachim6/hello-world (`v/Visual Basic for Applications.vba` at `a152253`),
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
    language: VisualBasicForApplications => "Visual Basic for Applications",
    versions: [VisualBasicForApplications => "Visual Basic for Applications"],
    extensions: [Vba => "vba"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasicForApplications,
        extensions: &[FileExtension::Vba],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VisualBasicForApplications,
            syntax: concat!(
                r#"Sub HelloWorld()
    Call MsgBox(""#,
                output_for_hello_world!(),
                r#"")
End Sub"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"Sub HelloWorld()
    Call MsgBox(""#,
                    output_for_hello_world!(),
                    r#"")
End Sub"#
                ),
            )],
        }],
    };

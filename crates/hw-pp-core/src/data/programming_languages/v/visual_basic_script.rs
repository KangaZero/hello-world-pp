//! Hello-world snippet for `ProgrammingLanguageName::VisualBasicScript`.
//!
//! Imported from leachim6/hello-world (`v/Visual Basic Script.vbs` at `a152253`),
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
    language: VisualBasicScript => "Visual Basic Script",
    versions: [VisualBasicScript => "Visual Basic Script"],
    extensions: [Vbs => "vbs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::VisualBasicScript,
        extensions: &[FileExtension::Vbs],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::VisualBasicScript,
            syntax: concat!(r#"MsgBox ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"MsgBox ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };

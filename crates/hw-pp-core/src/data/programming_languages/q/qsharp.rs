//! Hello-world snippet for `ProgrammingLanguageName::Qsharp`.
//!
//! Imported from leachim6/hello-world (`q/QSharp.qs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/q/QSharp.qs>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Qsharp => "QSharp",
    versions: [Qsharp => "QSharp"],
    extensions: [Qs => "qs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Qsharp,
        extensions: &[FileExtension::Qs],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Qsharp,
            syntax: concat!(
                "\u{feff}namespace Quantum.HelloWorld {\n    open Microsoft.Quantum.Canon;\n    open Microsoft.Quantum.Intrinsic;\n\n\n    operation HelloWorld() : Unit {\n        Message(\"",
                output_for_hello_world!(),
                r#"");
    }
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    "\u{feff}namespace Quantum.HelloWorld {\n    open Microsoft.Quantum.Canon;\n    open Microsoft.Quantum.Intrinsic;\n\n\n    operation HelloWorld() : Unit {\n        Message(\"",
                    output_for_hello_world!(),
                    r#"");
    }
}"#
                ),
            )],
        }],
    };

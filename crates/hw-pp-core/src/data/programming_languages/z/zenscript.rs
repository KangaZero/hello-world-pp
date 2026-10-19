//! Hello-world snippet for `ProgrammingLanguageName::Zenscript`.
//!
//! Imported from leachim6/hello-world (PR #1691 `z/zenscript.zs`),
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
    language: Zenscript => "ZenScript",
    versions: [Zenscript => "ZenScript"],
    extensions: [Zs => "zs"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zenscript,
        extensions: &[FileExtension::Zs],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zenscript,
            syntax: concat!(r#"println(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"println(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };

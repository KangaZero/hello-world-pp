//! Hello-world snippet for `ProgrammingLanguageName::Modiscript`.
//!
//! Imported from leachim6/hello-world (`m/ModiScript.chai` at `a152253`),
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
    language: Modiscript => "ModiScript",
    versions: [Modiscript => "ModiScript"],
    extensions: [Chai => "chai"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Modiscript,
        extensions: &[FileExtension::Chai],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Modiscript,
            syntax: concat!(
                r#"Mitrooon
    bhaiyo aur behno ""#,
                output_for_hello_world!(),
                r#""
Achhe din aa gaye"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"Mitrooon
    bhaiyo aur behno ""#,
                    output_for_hello_world!(),
                    r#""
Achhe din aa gaye"#
                ),
            )],
        }],
    };

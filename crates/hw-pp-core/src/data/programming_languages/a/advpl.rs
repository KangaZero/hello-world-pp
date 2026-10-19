//! Hello-world snippet for `ProgrammingLanguageName::Advpl`.
//!
//! Imported from leachim6/hello-world (`a/Advpl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Advpl => "Advpl",
    versions: [Advpl => "Advpl"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Advpl,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Advpl,
            syntax: concat!(
                r#"User Function Hello()
Local cMsg := ""#,
                output_for_hello_world!(),
                r#""
conout(cMsg)
MsgInfo(cMsg)
Return"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"User Function Hello()
Local cMsg := ""#,
                    output_for_hello_world!(),
                    r#""
conout(cMsg)
MsgInfo(cMsg)
Return"#
                ),
            )],
        }],
    };

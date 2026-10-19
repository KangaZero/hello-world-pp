//! Hello-world snippet for `ProgrammingLanguageName::Nwscript`.
//!
//! Imported from leachim6/hello-world (`n/NWScript.nss` at `a152253`),
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
    language: Nwscript => "NWScript",
    versions: [Nwscript => "NWScript"],
    extensions: [Nss => "nss"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nwscript,
        extensions: &[FileExtension::Nss],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nwscript,
            syntax: concat!(
                r#"// Place in the OnClientEnter event of the module's properties
{
    SendMessageToPC(GetEnteringObject(), ""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"// Place in the OnClientEnter event of the module's properties
{
    SendMessageToPC(GetEnteringObject(), ""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };

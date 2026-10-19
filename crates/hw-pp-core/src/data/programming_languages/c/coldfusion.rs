//! Hello-world snippet for `ProgrammingLanguageName::Coldfusion`.
//!
//! Imported from leachim6/hello-world (`c/ColdFusion.cfm` at `a152253`),
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
    language: Coldfusion => "ColdFusion",
    versions: [Coldfusion => "ColdFusion"],
    extensions: [Cfm => "cfm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Coldfusion,
        extensions: &[FileExtension::Cfm],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Coldfusion,
            syntax: concat!(
                r#"<cfset message = ""#,
                output_for_hello_world!(),
                r#"">
<cfoutput> #message#</cfoutput>"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"<cfset message = ""#,
                    output_for_hello_world!(),
                    r#"">
<cfoutput> #message#</cfoutput>"#
                ),
            )],
        }],
    };

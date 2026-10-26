//! Hello-world snippet for `ProgrammingLanguageName::Coldfusion`.
//!
//! Imported from leachim6/hello-world (`c/ColdFusion.cfm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/ColdFusion.cfm>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Coldfusion => "ColdFusion",
    versions: [Coldfusion => "ColdFusion"],
    extensions: [Cfm => "cfm"],
}

const COLDFUSION: &str = concat!(
    r#"<cfset message = ""#,
    output_for_hello_world!(),
    r#"">
<cfoutput> #message#</cfoutput>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Coldfusion,
        extensions: &[FileExtension::Cfm],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Coldfusion,
            syntax: COLDFUSION,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COLDFUSION,
                    [
                        (TokenKind::Keyword, r"<cfset"),
                        (TokenKind::Plain, r" message = "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Keyword, r">"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"<cfoutput>"),
                        (TokenKind::Plain, r" #message#"),
                        (TokenKind::Keyword, r"</cfoutput>"),
                    ],
                )
            },
        }],
    };

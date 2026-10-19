//! Hello-world snippet for `ProgrammingLanguageName::Dtrace`.
//!
//! Imported from leachim6/hello-world (`d/DTrace.d` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DTrace.d>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dtrace => "DTrace",
    versions: [Dtrace => "DTrace"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dtrace,
        extensions: &[],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dtrace,
            syntax: concat!(
                r#"#!/usr/sbin/dtrace -qs
BEGIN {
	printf(""#,
                output_for_hello_world!(),
                r#"");
	exit(0);
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#!/usr/sbin/dtrace -qs
BEGIN {
	printf(""#,
                    output_for_hello_world!(),
                    r#"");
	exit(0);
}"#
                ),
            )],
        }],
    };

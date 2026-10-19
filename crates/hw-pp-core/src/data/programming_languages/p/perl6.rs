//! Hello-world snippet for `ProgrammingLanguageName::Perl6`.
//!
//! Imported from leachim6/hello-world (`p/Perl6.p6` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Perl6.p6>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Perl6 => "Perl6",
    versions: [Perl6 => "Perl6"],
    extensions: [P6 => "p6"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perl6,
        extensions: &[FileExtension::P6],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Perl6,
            syntax: concat!(
                r#"#!/usr/bin/env perl6

v6;

say ""#,
                output_for_hello_world!(),
                r#"";
"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#!/usr/bin/env perl6

v6;

say ""#,
                    output_for_hello_world!(),
                    r#"";
"#
                ),
            )],
        }],
    };

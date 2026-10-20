//! Hello-world snippet for `ProgrammingLanguageName::Seed7`.
//!
//! Imported from leachim6/hello-world (`s/Seed7.s7` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Seed7.s7>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Seed7 => "Seed7",
    versions: [Seed7 => "Seed7"],
    extensions: [S7 => "s7"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Seed7,
        extensions: &[FileExtension::S7],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Seed7,
            syntax: concat!(
                r#"$ include "seed7_05.s7i";

const proc: main is func
  begin
    writeln(""#,
                output_for_hello_world!(),
                r#"");
  end func;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"$ include "seed7_05.s7i";

const proc: main is func
  begin
    writeln(""#,
                    output_for_hello_world!(),
                    r#"");
  end func;"#
                ),
            )],
        }],
    };

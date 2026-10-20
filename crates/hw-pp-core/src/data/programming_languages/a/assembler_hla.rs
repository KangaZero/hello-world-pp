//! Hello-world snippet for `ProgrammingLanguageName::AssemblerHla`.
//!
//! Imported from leachim6/hello-world (`a/Assembler hla.hla` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Assembler%20hla.hla>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerHla => "Assembler hla",
    versions: [AssemblerHla => "Assembler hla"],
    extensions: [Hla => "hla"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerHla,
        extensions: &[FileExtension::Hla],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerHla,
            syntax: concat!(
                r#"program HelloWorld;

#include( "stdlib.hhf" );

begin HelloWorld;
  stdout.put( ""#,
                output_for_hello_world!(),
                r#"", nl );
end HelloWorld;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"program HelloWorld;

#include( "stdlib.hhf" );

begin HelloWorld;
  stdout.put( ""#,
                    output_for_hello_world!(),
                    r#"", nl );
end HelloWorld;"#
                ),
            )],
        }],
    };

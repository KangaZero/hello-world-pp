//! Hello-world snippet for `ProgrammingLanguageName::Modula2`.
//!
//! Imported from leachim6/hello-world (`m/Modula 2.mod` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Modula%202.mod>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Modula2 => "Modula 2",
    versions: [Modula2 => "Modula 2"],
    extensions: [Mod => "mod"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Modula2,
        extensions: &[FileExtension::Mod],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Modula2,
            syntax: concat!(
                r#"MODULE HelloWorld;

FROM Terminal2 IMPORT WriteString, WriteLn;

BEGIN

    WriteString(""#,
                output_for_hello_world!(),
                r#"");
    WriteLn;
    
END HelloWorld."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"MODULE HelloWorld;

FROM Terminal2 IMPORT WriteString, WriteLn;

BEGIN

    WriteString(""#,
                    output_for_hello_world!(),
                    r#"");
    WriteLn;
    
END HelloWorld."#
                ),
            )],
        }],
    };

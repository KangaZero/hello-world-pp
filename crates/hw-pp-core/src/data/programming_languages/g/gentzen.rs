//! Hello-world snippet for `ProgrammingLanguageName::Gentzen`.
//!
//! Imported from leachim6/hello-world (`g/Gentzen.gentzen` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gentzen.gentzen>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Gentzen => "Gentzen",
    versions: [Gentzen => "Gentzen"],
    extensions: [Gentzen => "gentzen"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gentzen,
        extensions: &[FileExtension::Gentzen],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gentzen,
            syntax: concat!(
                r#"<- "stdlib";
<- "iolib";
main : () / ($1(iosys) |- $2(++));
main = [x,y] cut ""#,
                output_for_hello_world!(),
                r#"" [z] outtext x z y;
-> main;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"<- "stdlib";
<- "iolib";
main : () / ($1(iosys) |- $2(++));
main = [x,y] cut ""#,
                    output_for_hello_world!(),
                    r#"" [z] outtext x z y;
-> main;"#
                ),
            )],
        }],
    };

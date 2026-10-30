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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Gentzen => "Gentzen",
    versions: [Gentzen => "Gentzen"],
    extensions: [Gentzen => "gentzen"],
}

const GENTZEN: &str = concat!(
    r#"<- "stdlib";
<- "iolib";
main : () / ($1(iosys) |- $2(++));
main = [x,y] cut ""#,
    output_for_hello_world!(),
    r#"" [z] outtext x z y;
-> main;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gentzen,
        extensions: &[FileExtension::Gentzen],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gentzen,
            syntax: GENTZEN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GENTZEN,
                    [(
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
                )
            },
        }],
    };

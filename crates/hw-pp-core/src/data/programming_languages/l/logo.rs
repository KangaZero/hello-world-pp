//! Hello-world snippets for `ProgrammingLanguageName::Logo`.
//!
//! Imported from leachim6/hello-world (`l/Logo.lg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/Logo&nbsp;(graphical).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Logo.lg>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Logo%26nbsp%3B%28graphical%29.txt>
//! - <http://helloworldcollection.de/#Logo%C2%A0(graphical)>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{Lisp, Logo},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Logo => "Logo",
    versions: [Logo => "Logo", LogoGraphical => "Logo (graphical)"],
    extensions: [Lg => "lg"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Logo,
        extensions: &[FileExtension::Lg],
        categories: &[Educational],
        families: &[Lisp, Logo],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Logo,
                syntax: concat!(r"print [", output_for_hello_world!(), r"]"),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(r"print [", output_for_hello_world!(), r"]"),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::LogoGraphical,
                syntax: concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r" in LOGO, graphical output.

go 20 , left 180,
go 40 , left 180,
go 20 , right 90,
go 20 , left 90 ,
go 20 , left 180,
go 40 , left 90 ,
go 20 , left 90 ,
go 20 , right 90 ,
go 20 , right 90 ,
go 10 , right 90 ,
go 20 , left 90 ,
go 10 , left 90 ,
go 30 , left 90 ,
go 40 , left 180,
go 40 , left 90 ,
go 20 , left 90 ,
go 40 , left 180,
go 40 , left 90 ,
go 40 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 60 , left 90 ,
go 40 , left 180,
go 40 , left 90 ,
go 20 , left 90 ,
go 20 , left 180,
go 20 , left 90 ,
go 20 , left 90 ,
go 40 , left 180,
go 40 , left 90 ,
go 40 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 40 , left 90 ,
go 20 , right 90,
go 20 , right 90,
go 5 , left 90  ,
go 5 , left 90  ,
go 25 , left 180,
go 40 , left 90 ,
go 40 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 40 , left 180,
go 40 ,"
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"; ",
                        output_for_hello_world!(),
                        r" in LOGO, graphical output.

go 20 , left 180,
go 40 , left 180,
go 20 , right 90,
go 20 , left 90 ,
go 20 , left 180,
go 40 , left 90 ,
go 20 , left 90 ,
go 20 , right 90 ,
go 20 , right 90 ,
go 10 , right 90 ,
go 20 , left 90 ,
go 10 , left 90 ,
go 30 , left 90 ,
go 40 , left 180,
go 40 , left 90 ,
go 20 , left 90 ,
go 40 , left 180,
go 40 , left 90 ,
go 40 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 60 , left 90 ,
go 40 , left 180,
go 40 , left 90 ,
go 20 , left 90 ,
go 20 , left 180,
go 20 , left 90 ,
go 20 , left 90 ,
go 40 , left 180,
go 40 , left 90 ,
go 40 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 40 , left 90 ,
go 20 , right 90,
go 20 , right 90,
go 5 , left 90  ,
go 5 , left 90  ,
go 25 , left 180,
go 40 , left 90 ,
go 40 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 20 , left 90 ,
go 40 , left 180,
go 40 ,"
                    ),
                )],
            },
        ],
    };

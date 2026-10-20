//! Hello-world snippet for `ProgrammingLanguageName::XbasePlusPlus`.
//!
//! Imported from leachim6/hello-world (`x/XBase++.prg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XBase%2B%2B.prg>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: XbasePlusPlus => "XBase++",
    versions: [XbasePlusPlus => "XBase++"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::XbasePlusPlus,
        extensions: &[],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::XbasePlusPlus,
            syntax: concat!(
                r#"func Main()
    Qout(""#,
                output_for_hello_world!(),
                r#"")
return 1"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"func Main()
    Qout(""#,
                    output_for_hello_world!(),
                    r#"")
return 1"#
                ),
            )],
        }],
    };

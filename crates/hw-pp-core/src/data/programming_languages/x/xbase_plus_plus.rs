//! Hello-world snippet for `ProgrammingLanguageName::XbasePlusPlus`.
//!
//! Imported from leachim6/hello-world (`x/XBase++.prg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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

//! Hello-world snippet for `ProgrammingLanguageName::LuaGrimex`.
//!
//! Imported from leachim6/hello-world (`l/Lua Grimex.j` at `a152253`),
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
    language: LuaGrimex => "Lua Grimex",
    versions: [LuaGrimex => "Lua Grimex"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LuaGrimex,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LuaGrimex,
            syntax: concat!(
                r#"//! externalblock extension=lua ObjectMerger $FILENAME$

//! i   log(""#,
                output_for_hello_world!(),
                r#"")

//! endexternalblock"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"//! externalblock extension=lua ObjectMerger $FILENAME$

//! i   log(""#,
                    output_for_hello_world!(),
                    r#"")

//! endexternalblock"#
                ),
            )],
        }],
    };

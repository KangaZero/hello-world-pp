//! Hello-world snippet for `ProgrammingLanguageName::LuaGrimex`.
//!
//! Imported from leachim6/hello-world (`l/Lua Grimex.j` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Lua%20Grimex.j>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: LuaGrimex => "Lua Grimex",
    versions: [LuaGrimex => "Lua Grimex"],
    extensions: [],
}

const LUA_GRIMEX: &str = concat!(
    r#"//! externalblock extension=lua ObjectMerger $FILENAME$

//! i   log(""#,
    output_for_hello_world!(),
    r#"")

//! endexternalblock"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LuaGrimex,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LuaGrimex,
            syntax: LUA_GRIMEX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LUA_GRIMEX,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"//! externalblock extension=lua ObjectMerger $FILENAME$

//! i   log(""#,
                            output_for_hello_world!(),
                            r#"")

//! endexternalblock"#
                        ),
                    )],
                )
            },
        }],
    };

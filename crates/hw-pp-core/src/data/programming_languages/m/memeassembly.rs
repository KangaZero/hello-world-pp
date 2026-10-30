//! Hello-world snippet for `ProgrammingLanguageName::Memeassembly`.
//!
//! Imported from leachim6/hello-world (`m/MemeAssembly.memeasm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/MemeAssembly.memeasm>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Memeassembly => "MemeAssembly",
    versions: [Memeassembly => "MemeAssembly"],
    extensions: [Memeasm => "memeasm"],
}

const MEMEASSEMBLY: &str = r"I like to have fun, fun, fun, fun, fun, fun, fun, fun, fun, fun main
    what can I say except H
    what can I say except e
    what can I say except l
    what can I say except l
    what can I say except o
    what can I say except \s
    what can I say except W
    what can I say except o
    what can I say except r
    what can I say except l
    what can I say except d
    what can I say except \n

    I see this as an absolute win";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Memeassembly,
        extensions: &[FileExtension::Memeasm],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Memeassembly,
            syntax: MEMEASSEMBLY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MEMEASSEMBLY,
                    [(
                        TokenKind::Plain,
                        r"I like to have fun, fun, fun, fun, fun, fun, fun, fun, fun, fun main
    what can I say except H
    what can I say except e
    what can I say except l
    what can I say except l
    what can I say except o
    what can I say except \s
    what can I say except W
    what can I say except o
    what can I say except r
    what can I say except l
    what can I say except d
    what can I say except \n

    I see this as an absolute win",
                    )],
                )
            },
        }],
    };

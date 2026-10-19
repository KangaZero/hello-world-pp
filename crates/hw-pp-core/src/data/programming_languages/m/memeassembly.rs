//! Hello-world snippet for `ProgrammingLanguageName::Memeassembly`.
//!
//! Imported from leachim6/hello-world (`m/MemeAssembly.memeasm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Memeassembly => "MemeAssembly",
    versions: [Memeassembly => "MemeAssembly"],
    extensions: [Memeasm => "memeasm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Memeassembly,
        extensions: &[FileExtension::Memeasm],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Memeassembly,
            syntax: r"I like to have fun, fun, fun, fun, fun, fun, fun, fun, fun, fun main
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
            tokenized_syntax: &[(
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
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Hodor`.
//!
//! Imported from leachim6/hello-world (`h/Hodor.hd` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Hodor.hd>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Hodor => "Hodor",
    versions: [Hodor => "Hodor"],
    extensions: [Hd => "hd"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hodor,
        extensions: &[FileExtension::Hd],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hodor,
            syntax: r"hodor.hod('Hhodor? Hodor!? Hodor!? o, Hooodorrhodor orHodor!? d!');",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"hodor.hod('Hhodor? Hodor!? Hodor!? o, Hooodorrhodor orHodor!? d!');",
            )],
        }],
    };

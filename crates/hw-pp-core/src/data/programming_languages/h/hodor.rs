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
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Hodor => "Hodor",
    versions: [Hodor => "Hodor"],
    extensions: [Hd => "hd"],
}

const HODOR: &str = r"hodor.hod('Hhodor? Hodor!? Hodor!? o, Hooodorrhodor orHodor!? d!');";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hodor,
        extensions: &[FileExtension::Hd],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hodor,
            syntax: HODOR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HODOR,
                    [(
                        TokenKind::Plain,
                        r"hodor.hod('Hhodor? Hodor!? Hodor!? o, Hooodorrhodor orHodor!? d!');",
                    )],
                )
            },
        }],
    };

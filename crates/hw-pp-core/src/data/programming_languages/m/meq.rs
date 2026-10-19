//! Hello-world snippet for `ProgrammingLanguageName::Meq`.
//!
//! Imported from leachim6/hello-world (`m/Meq.meq` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Meq.meq>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Meq => "Meq",
    versions: [Meq => "Meq"],
    extensions: [Meq => "meq"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Meq,
        extensions: &[FileExtension::Meq],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Meq,
            syntax: r".==++>:+>+[>:++++++++]>:=+>===++>.===+++++++>:=+>:=++++>:++++++++>:rp>p>>p>p>p>p>p>p>p>p>p!",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r".==++>:+>+[>:++++++++]>:=+>===++>.===+++++++>:=+>:=++++>:++++++++>:rp>p>>p>p>p>p>p>p>p>p>p!",
            )],
        }],
    };

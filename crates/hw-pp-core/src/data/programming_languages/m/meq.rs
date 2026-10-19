//! Hello-world snippet for `ProgrammingLanguageName::Meq`.
//!
//! Imported from leachim6/hello-world (`m/Meq.meq` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Meq,
            syntax: r".==++>:+>+[>:++++++++]>:=+>===++>.===+++++++>:=+>:=++++>:++++++++>:rp>p>>p>p>p>p>p>p>p>p>p!",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r".==++>:+>+[>:++++++++]>:=+>===++>.===+++++++>:=+>:=++++>:++++++++>:rp>p>>p>p>p>p>p>p>p>p>p!",
            )],
        }],
    };

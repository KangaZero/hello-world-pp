//! Hello-world snippet for `ProgrammingLanguageName::Omgrofl`.
//!
//! Imported from leachim6/hello-world (`o/Omgrofl.omgrofl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Omgrofl => "Omgrofl",
    versions: [Omgrofl => "Omgrofl"],
    extensions: [Omgrofl => "omgrofl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Omgrofl,
        extensions: &[FileExtension::Omgrofl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Omgrofl,
            syntax: r"lol iz 72
rofl lol
lol iz 101
rofl lol
lol iz 108
rofl lol
rofl lol
lool iz 111
rofl lool
loool iz 44
rofl loool
loool iz 32
rofl loool
loool iz 87
rofl loool
rofl lool
lool iz 114
rofl lool
rofl lol
lol iz 100
rofl lol
lol iz 33
rofl lol",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"lol iz 72
rofl lol
lol iz 101
rofl lol
lol iz 108
rofl lol
rofl lol
lool iz 111
rofl lool
loool iz 44
rofl loool
loool iz 32
rofl loool
loool iz 87
rofl loool
rofl lool
lool iz 114
rofl lool
rofl lol
lol iz 100
rofl lol
lol iz 33
rofl lol",
            )],
        }],
    };

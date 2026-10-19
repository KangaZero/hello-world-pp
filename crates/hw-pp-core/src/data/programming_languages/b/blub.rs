//! Hello-world snippet for `ProgrammingLanguageName::Blub`.
//!
//! Imported from leachim6/hello-world (`b/Blub.blub` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Blub => "Blub",
    versions: [Blub => "Blub"],
    extensions: [Blub => "blub"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blub,
        extensions: &[FileExtension::Blub],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blub,
            syntax: r"blub. blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub! blub?
blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub? blub! blub!
blub? blub! blub? blub. blub! blub. blub. blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub.
blub! blub? blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub? blub! blub! blub? blub! blub? blub. blub. blub.
blub! blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub! blub. blub! blub. blub. blub.
blub. blub. blub. blub. blub! blub. blub. blub? blub. blub? blub. blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub.
blub. blub. blub. blub. blub. blub. blub! blub? blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub? blub! blub!
blub? blub! blub? blub. blub! blub. blub. blub? blub. blub? blub. blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub.
blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub! blub? blub? blub. blub. blub. blub. blub. blub. blub. blub. blub.
blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub? blub! blub! blub? blub! blub? blub. blub! blub! blub! blub!
blub! blub! blub! blub. blub? blub. blub? blub. blub? blub. blub? blub. blub! blub. blub. blub. blub. blub. blub. blub. blub! blub.
blub! blub! blub! blub! blub! blub! blub! blub! blub! blub! blub! blub! blub! blub. blub! blub! blub! blub! blub! blub! blub! blub!
blub! blub! blub! blub! blub! blub! blub! blub! blub! blub. blub. blub? blub. blub? blub. blub. blub! blub.",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"blub. blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub! blub?
blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub? blub! blub!
blub? blub! blub? blub. blub! blub. blub. blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub.
blub! blub? blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub? blub! blub! blub? blub! blub? blub. blub. blub.
blub! blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub! blub. blub! blub. blub. blub.
blub. blub. blub. blub. blub! blub. blub. blub? blub. blub? blub. blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub.
blub. blub. blub. blub. blub. blub. blub! blub? blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub? blub! blub!
blub? blub! blub? blub. blub! blub. blub. blub? blub. blub? blub. blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub.
blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub! blub? blub? blub. blub. blub. blub. blub. blub. blub. blub. blub.
blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub? blub! blub! blub? blub! blub? blub. blub! blub! blub! blub!
blub! blub! blub! blub. blub? blub. blub? blub. blub? blub. blub? blub. blub! blub. blub. blub. blub. blub. blub. blub. blub! blub.
blub! blub! blub! blub! blub! blub! blub! blub! blub! blub! blub! blub! blub! blub. blub! blub! blub! blub! blub! blub! blub! blub!
blub! blub! blub! blub! blub! blub! blub! blub! blub! blub. blub. blub? blub. blub? blub. blub. blub! blub.",
            )],
        }],
    };

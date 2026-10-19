//! Hello-world snippet for `ProgrammingLanguageName::AtTentionBang`.
//!
//! Imported from leachim6/hello-world (`#/@tention!` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%40tention%21>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: AtTentionBang => "@tention!",
    versions: [AtTentionBang => "@tention!"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AtTentionBang,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AtTentionBang,
            syntax: r"A@=;AH'<;Ae'<;Al'<;Al'<;Ao'<;A '<;AW'<;Ao'<;Ar'<;Al'<;Ad'<;A!{A$>};",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"A@=;AH'<;Ae'<;Al'<;Al'<;Ao'<;A '<;AW'<;Ao'<;Ar'<;Al'<;Ad'<;A!{A$>};",
            )],
        }],
    };

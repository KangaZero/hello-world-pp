//! Hello-world snippet for `ProgrammingLanguageName::Blub`.
//!
//! Imported from leachim6/hello-world (`b/Blub.blub` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Blub.blub>
//! - <https://esolangs.org/wiki/Blub> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Blub => "Blub",
    versions: [Blub => "Blub"],
    extensions: [Blub => "blub"],
}

const BLUB: &str = r"blub. blub? blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub. blub! blub?
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
blub! blub! blub! blub! blub! blub! blub! blub! blub! blub. blub. blub? blub. blub? blub. blub. blub! blub.";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blub,
        extensions: &[FileExtension::Blub],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blub,
            syntax: BLUB,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BLUB,
                    [(
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
                )
            },
        }],
    };

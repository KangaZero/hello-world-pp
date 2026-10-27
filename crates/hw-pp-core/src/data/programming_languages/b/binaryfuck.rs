//! Hello-world snippet for `ProgrammingLanguageName::Binaryfuck`.
//!
//! Imported from leachim6/hello-world (`b/Binaryfuck.bf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Binaryfuck.bf>
//! - <https://esolangs.org/wiki/Binaryfuck> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Binaryfuck => "Binaryfuck",
    versions: [Binaryfuck => "Binaryfuck"],
    extensions: [],
}

const BINARYFUCK: &str = r"000 000 000 000 000 000 000 000 000 000 110 010 000 010 000
000 000 010 000 000 000 000 000 000 000 010 000 000 000 000
000 000 000 000 000 000 011 011 011 011 001 111 010 010 010
000 000 100 010 000 100 000 000 000 000 000 000 000 100 100
000 000 000 100 011 011 000 000 100 010 000 000 000 000 000
000 000 000 000 000 000 000 000 000 000 100 010 100 000 000
000 100 001 001 001 001 001 001 100 001 001 001 001 001 001
001 001 100";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Binaryfuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Binaryfuck,
            syntax: BINARYFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BINARYFUCK,
                    [(
                        TokenKind::Plain,
                        r"000 000 000 000 000 000 000 000 000 000 110 010 000 010 000
000 000 010 000 000 000 000 000 000 000 010 000 000 000 000
000 000 000 000 000 000 011 011 011 011 001 111 010 010 010
000 000 100 010 000 100 000 000 000 000 000 000 000 100 100
000 000 000 100 011 011 000 000 100 010 000 000 000 000 000
000 000 000 000 000 000 000 000 000 000 100 010 100 000 000
000 100 001 001 001 001 001 001 100 001 001 001 001 001 001
001 001 100",
                    )],
                )
            },
        }],
    };

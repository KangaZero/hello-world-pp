//! Hello-world snippet for `ProgrammingLanguageName::N123`.
//!
//! Imported from leachim6/hello-world (`#/123` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/123>
//! - <https://esolangs.org/wiki/123> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: N123 => "123",
    versions: [N123 => "123"],
    extensions: [],
}

const N123: &str = r"2221222212121121121121121121121122221222212121121121121121121
222222111211211121121211222222211121121112112
2222221112111121121122222221112111121
2222221112111121121122222221112111121
2222221112111112121122222221112111112
222221111112222211111122222211111122222111112
2222221121112111112222222112111211112
2222221112111112121122222221112111112
2222221111211211121122222221111211211
2222221112111121121122222221112111121
22222211121121112112112222222111211211121";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N123,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N123,
            syntax: N123,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N123,
                    [(
                        TokenKind::Plain,
                        r"2221222212121121121121121121121122221222212121121121121121121
222222111211211121121211222222211121121112112
2222221112111121121122222221112111121
2222221112111121121122222221112111121
2222221112111112121122222221112111112
222221111112222211111122222211111122222111112
2222221121112111112222222112111211112
2222221112111112121122222221112111112
2222221111211211121122222221111211211
2222221112111121121122222221112111121
22222211121121112112112222222111211211121",
                    )],
                )
            },
        }],
    };

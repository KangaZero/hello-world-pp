//! Hello-world snippet for `ProgrammingLanguageName::Semicolon`.
//!
//! Imported from leachim6/hello-world (`s/Semicolon` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Semicolon>
//! - <https://esolangs.org/wiki/Semicolon> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Semicolon => "Semicolon",
    versions: [Semicolon => "Semicolon"],
    extensions: [],
}

const SEMICOLON: &str = r";;;;⁏;;⁏;;;
⁏ ;;;;;;⁏⁏;;⁏;⁏
⁏ ;;;;;;⁏⁏;⁏⁏;;
;;⁏⁏ ;;⁏ ;;;;;;⁏⁏;⁏⁏⁏⁏
⁏ ;;;;;;⁏;;;;;
⁏ ;;;;;;⁏⁏⁏;⁏⁏⁏
⁏ ;;;;;;⁏⁏;⁏⁏⁏⁏
;;⁏⁏ ;;;;;;⁏⁏
⁏;;⁏ ;;;;;;⁏⁏;⁏⁏;;
⁏ ;;;;;;⁏⁏;;⁏;;
⁏ ;;;;;;⁏;;;;⁏
⁏ ;;;;;;;⁏;⁏;
⁏ ;;  ;";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Semicolon,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Semicolon,
            syntax: SEMICOLON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SEMICOLON,
                    [(
                        TokenKind::Plain,
                        r";;;;⁏;;⁏;;;
⁏ ;;;;;;⁏⁏;;⁏;⁏
⁏ ;;;;;;⁏⁏;⁏⁏;;
;;⁏⁏ ;;⁏ ;;;;;;⁏⁏;⁏⁏⁏⁏
⁏ ;;;;;;⁏;;;;;
⁏ ;;;;;;⁏⁏⁏;⁏⁏⁏
⁏ ;;;;;;⁏⁏;⁏⁏⁏⁏
;;⁏⁏ ;;;;;;⁏⁏
⁏;;⁏ ;;;;;;⁏⁏;⁏⁏;;
⁏ ;;;;;;⁏⁏;;⁏;;
⁏ ;;;;;;⁏;;;;⁏
⁏ ;;;;;;;⁏;⁏;
⁏ ;;  ;",
                    )],
                )
            },
        }],
    };

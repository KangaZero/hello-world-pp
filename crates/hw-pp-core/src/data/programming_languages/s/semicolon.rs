//! Hello-world snippet for `ProgrammingLanguageName::Semicolon`.
//!
//! Imported from leachim6/hello-world (`s/Semicolon` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Semicolon>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Semicolon => "Semicolon",
    versions: [Semicolon => "Semicolon"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Semicolon,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Semicolon,
            syntax: r";;;;⁏;;⁏;;;
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
            tokenized_syntax: &[(
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
        }],
    };

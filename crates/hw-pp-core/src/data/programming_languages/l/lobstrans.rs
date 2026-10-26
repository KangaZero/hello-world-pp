//! Hello-world snippet for `ProgrammingLanguageName::Lobstrans`.
//!
//! Lobstrans has no way to print "Hello, World!"; 🎌 is the only command that greets at all.
//! Reading the reference interpreter, it prints:
//!
//! ```text
//! こんにちは!
//! こ
//! ん
//! に
//! ち
//! は
//! !
//! Japan double flag
//! 🎌
//! ```
//!
//! and then crashes: the spec says 🎌 enqueues itself, but the interpreter calls
//! `q.queue("🎌")` on the queue's `deque`, which raises `TypeError`.
//!
//! The language is released into the public domain (The Unlicense).
//!
//! Sources:
//! - <https://codeberg.org/de67/lobstrans/src/commit/a93a8b8ffc6ddd4dc823d7a52dedf76522e15a12/interpreter.py>
//! - <https://codeberg.org/de67/lobstrans/src/commit/a93a8b8ffc6ddd4dc823d7a52dedf76522e15a12/SPEC.md>

use crate::data::macros::language_names;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

language_names! {
    language: Lobstrans => "Lobstrans",
    versions: [Lobstrans => "Lobstrans"],
    extensions: [],
}

const LOBSTRANS: &str = "🎌";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lobstrans,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lobstrans,
            syntax: LOBSTRANS,
            tokenized_syntax: &const {
                checked_token_lengths(LOBSTRANS, [(TokenKind::Keyword, LOBSTRANS)])
            },
        }],
    };

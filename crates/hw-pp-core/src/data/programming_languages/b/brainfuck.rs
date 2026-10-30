//! Hello-world snippet for `ProgrammingLanguageName::Brainfuck`.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Brainfuck>
//! - <https://en.wikipedia.org/wiki/Brainfuck>
//! - <https://www.wikidata.org/wiki/Q244627> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, Imperative};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Brainfuck => "Brainfuck",
    versions: [Brainfuck => "Brainfuck"],
    extensions: [Bf => "bf"],
}

const BRAINFUCK: &str = r"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainfuck,
        extensions: &[FileExtension::Bf],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainfuck,
            syntax: BRAINFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRAINFUCK,
                    [
                        (TokenKind::Number, "++++++++"),
                        (TokenKind::Punctuation, "["),
                        (TokenKind::Keyword, ">"),
                        (TokenKind::Number, "++++"),
                        (TokenKind::Punctuation, "["),
                        (TokenKind::Keyword, ">"),
                        (TokenKind::Number, "++"),
                        (TokenKind::Keyword, ">"),
                        (TokenKind::Number, "+++"),
                        (TokenKind::Keyword, ">"),
                        (TokenKind::Number, "+++"),
                        (TokenKind::Keyword, ">"),
                        (TokenKind::Number, "+"),
                        (TokenKind::Keyword, "<<<<"),
                        (TokenKind::Number, "-"),
                        (TokenKind::Punctuation, "]"),
                        (TokenKind::Keyword, ">"),
                        (TokenKind::Number, "+"),
                        (TokenKind::Keyword, ">"),
                        (TokenKind::Number, "+"),
                        (TokenKind::Keyword, ">"),
                        (TokenKind::Number, "-"),
                        (TokenKind::Keyword, ">>"),
                        (TokenKind::Number, "+"),
                        (TokenKind::Punctuation, "["),
                        (TokenKind::Keyword, "<"),
                        (TokenKind::Punctuation, "]"),
                        (TokenKind::Keyword, "<"),
                        (TokenKind::Number, "-"),
                        (TokenKind::Punctuation, "]"),
                        (TokenKind::Keyword, ">>"),
                        (TokenKind::Function, "."),
                        (TokenKind::Keyword, ">"),
                        (TokenKind::Plain, "\n"),
                        (TokenKind::Number, "---"),
                        (TokenKind::Function, "."),
                        (TokenKind::Number, "+++++++"),
                        (TokenKind::Function, ".."),
                        (TokenKind::Number, "+++"),
                        (TokenKind::Function, "."),
                        (TokenKind::Keyword, ">>"),
                        (TokenKind::Function, "."),
                        (TokenKind::Keyword, "<"),
                        (TokenKind::Number, "-"),
                        (TokenKind::Function, "."),
                        (TokenKind::Keyword, "<"),
                        (TokenKind::Function, "."),
                        (TokenKind::Number, "+++"),
                        (TokenKind::Function, "."),
                        (TokenKind::Number, "------"),
                        (TokenKind::Function, "."),
                        (TokenKind::Number, "--------"),
                        (TokenKind::Function, "."),
                        (TokenKind::Keyword, ">>"),
                        (TokenKind::Number, "+"),
                        (TokenKind::Function, "."),
                        (TokenKind::Keyword, ">"),
                        (TokenKind::Number, "++"),
                        (TokenKind::Function, "."),
                    ],
                )
            },
        }],
    };

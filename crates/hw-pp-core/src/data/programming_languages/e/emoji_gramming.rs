//! Hello-world snippet for `ProgrammingLanguageName::EmojiGramming`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Emoji-gramming>
//! - <https://esolangs.org/wiki/Emoji-gramming>

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: EmojiGramming => "Emoji-gramming",
    versions: [EmojiGramming => "Emoji-gramming"],
    extensions: [],
}

const EMOJI_GRAMMING: &str = r"😊🕐💖
😇🕐🕐
😊🕑🕐
😇🕑🕑
😊🕒🕑
😇🕒🕒
😊♈💖
😇♈🕒
😊📢♈
😊♈💜
😇♈💞
😇♈🕑
😇♈🕒
😊📢♈
😊♉💞
😇♉💖
😇♉🕑
😇♉🕒
😊📢♉
😊📢♉
😊♊💜
😇♊💕
😇♊💞
😇♊💖
😇♊🕑
😇♊🕒
😊📢♊
😊♈💞
😇♈💖
😇♈🕑
😊📢♈
😊📢🕑
😊♈💜
😇♈💕
😇♈💞
😇♈🕐
😇♈🕒
😊📢♈
😊📢♊
😊♈💕
😇♈🕐
😇♈🕑
😇♈🕒
😊📢♈
😊📢♉
😊♈💞
😇♈🕑
😇♈🕒
😊📢♈
😊♈💜
😇♈🕑
😊📢♈
😊♈💖
😇♈💕
😊📢♈";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::EmojiGramming,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::EmojiGramming,
            syntax: EMOJI_GRAMMING,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EMOJI_GRAMMING,
                    [(
                        TokenKind::Plain,
                        r"😊🕐💖
😇🕐🕐
😊🕑🕐
😇🕑🕑
😊🕒🕑
😇🕒🕒
😊♈💖
😇♈🕒
😊📢♈
😊♈💜
😇♈💞
😇♈🕑
😇♈🕒
😊📢♈
😊♉💞
😇♉💖
😇♉🕑
😇♉🕒
😊📢♉
😊📢♉
😊♊💜
😇♊💕
😇♊💞
😇♊💖
😇♊🕑
😇♊🕒
😊📢♊
😊♈💞
😇♈💖
😇♈🕑
😊📢♈
😊📢🕑
😊♈💜
😇♈💕
😇♈💞
😇♈🕐
😇♈🕒
😊📢♈
😊📢♊
😊♈💕
😇♈🕐
😇♈🕑
😇♈🕒
😊📢♈
😊📢♉
😊♈💞
😇♈🕑
😇♈🕒
😊📢♈
😊♈💜
😇♈🕑
😊📢♈
😊♈💖
😇♈💕
😊📢♈",
                    )],
                )
            },
        }],
    };

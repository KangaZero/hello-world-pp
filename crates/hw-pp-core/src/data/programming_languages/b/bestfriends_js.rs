//! Hello-world snippet for `ProgrammingLanguageName::BestfriendsJs`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#BestFriends.js>
//! - <https://esolangs.org/wiki/BestFriends.js>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: BestfriendsJs => "BestFriends.js",
    versions: [BestfriendsJs => "BestFriends.js"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BestfriendsJs,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BestfriendsJs,
            syntax: r#"var c = "+++++ [>+++++<-] > + }|" +
        "[<++++>-]<.[>+>+<<-]}{|" +
        ">++++<---.>..+++.@  {>";
alert(BF(c,"input") + " world");"#,
            tokenized_syntax: &[(
                TokenKind::Plain,
                r#"var c = "+++++ [>+++++<-] > + }|" +
        "[<++++>-]<.[>+>+<<-]}{|" +
        ">++++<---.>..+++.@  {>";
alert(BF(c,"input") + " world");"#,
            )],
        }],
    };

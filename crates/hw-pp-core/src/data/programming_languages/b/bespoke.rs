//! Hello-world snippet for `ProgrammingLanguageName::Bespoke`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Bespoke>
//! - <https://esolangs.org/wiki/Bespoke>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Bespoke => "Bespoke",
    versions: [Bespoke => "Bespoke"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bespoke,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bespoke,
            syntax: r"more peppermint tea?
ah yes, it's not bad
I appreciate peppermint tea
it's a refreshing beverage

but you immediately must try the gingerbread
I had it sometime, forever ago
oh, and it was so good!
made the way a gingerbread must clearly be baked

in fact, I've got a suggestion
I may go outside
to Marshal Mellow's Bakery
so we both receive one",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"more peppermint tea?
ah yes, it's not bad
I appreciate peppermint tea
it's a refreshing beverage

but you immediately must try the gingerbread
I had it sometime, forever ago
oh, and it was so good!
made the way a gingerbread must clearly be baked

in fact, I've got a suggestion
I may go outside
to Marshal Mellow's Bakery
so we both receive one",
            )],
        }],
    };

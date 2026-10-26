//! Hello-world snippet for `ProgrammingLanguageName::ColorDialog`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Color_Dialog>
//! - <https://esolangs.org/wiki/Color_Dialog>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: ColorDialog => "Color Dialog",
    versions: [ColorDialog => "Color Dialog"],
    extensions: [],
}

const COLOR_DIALOG: &str = r"㢀You must㶀뼢 construc㸀㞀
㢀t addition㶀뼢al pylons 꽱b㸀㞀
㢀efore the㶀뼢 swarm arriv㸀㞀
㢀es at the㶀뼢 northern ou㸀㞀
㢀tpost, comm㶀뼢ander, and꽱 㸀㞀
㢀hold㶀뼢 that ridge㸀㞀
㢀 unt㶀뼢il the f㸀㞀
㢀leet has 㶀뼢landed on 쩢the㸀㞀
㢀 plateau be㶀뼢yond the r꽱u㸀㞀
㢀ined colony㶀뼢 gates at 꽱dawn㸀㞀
㢀 today, o㶀뼢r every last㸀㞀
㢀 marine th㶀뼢ere is los㸀㞀
㢀t t㶀뼢o the tides㸀㞀";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ColorDialog,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ColorDialog,
            syntax: COLOR_DIALOG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COLOR_DIALOG,
                    [(
                        TokenKind::Plain,
                        r"㢀You must㶀뼢 construc㸀㞀
㢀t addition㶀뼢al pylons 꽱b㸀㞀
㢀efore the㶀뼢 swarm arriv㸀㞀
㢀es at the㶀뼢 northern ou㸀㞀
㢀tpost, comm㶀뼢ander, and꽱 㸀㞀
㢀hold㶀뼢 that ridge㸀㞀
㢀 unt㶀뼢il the f㸀㞀
㢀leet has 㶀뼢landed on 쩢the㸀㞀
㢀 plateau be㶀뼢yond the r꽱u㸀㞀
㢀ined colony㶀뼢 gates at 꽱dawn㸀㞀
㢀 today, o㶀뼢r every last㸀㞀
㢀 marine th㶀뼢ere is los㸀㞀
㢀t t㶀뼢o the tides㸀㞀",
                    )],
                )
            },
        }],
    };

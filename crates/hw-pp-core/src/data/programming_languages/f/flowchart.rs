//! Hello-world snippet for `ProgrammingLanguageName::Flowchart`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Flowchart>
//! - <https://esolangs.org/wiki/Flowchart>

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, TwoDimensional};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Flowchart => "Flowchart",
    versions: [Flowchart => "Flowchart"],
    extensions: [],
}

const FLOWCHART: &str = r"     ┌──{ ]─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \───┐
 ┌──( )─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \───┤
 │   └──{ ]─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─{ ]─\ \───┤
 │   ┌──[ }─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \───┤
( )─( )─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \───┤
 │   └──{ ]─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \───┤
 └──( )─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─{ ]─\ \───┤
     └──{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─(( ))";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flowchart,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flowchart,
            syntax: FLOWCHART,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FLOWCHART,
                    [(
                        TokenKind::Plain,
                        r"     ┌──{ ]─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \───┐
 ┌──( )─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \───┤
 │   └──{ ]─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─{ ]─\ \───┤
 │   ┌──[ }─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \───┤
( )─( )─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \───┤
 │   └──{ ]─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \───┤
 └──( )─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─{ ]─\ \─{ ]─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─[ }─\ \─{ ]─\ \───┤
     └──{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─{ ]─\ \─(( ))",
                    )],
                )
            },
        }],
    };

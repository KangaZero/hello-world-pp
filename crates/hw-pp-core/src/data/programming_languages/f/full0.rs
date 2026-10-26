//! Hello-world snippet for `ProgrammingLanguageName::Full0`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Full_0>
//! - <https://esolangs.org/wiki/Full_0>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Full0 => "Full 0",
    versions: [Full0 => "Full 0"],
    extensions: [],
}

const FULL0: &str = r"{6}[(a0)(?H)]{3}[a0]{2}[(a0)(?e)]{3}[a0]
{2}[(a0)(?l)]{3}[a0]~{1}[a0]~{4}[(a0)(?o)]
{3}[a0]{2}[(a0)(?,)]{3}[a0]{2}[(a0)(? )]
{3}[a0]{2}[(a0)(?w)]{3}[a0]{2}[(a0)(?o)]
{3}[a0]{2}[(a0)(?r)]{3}[a0]{2}[(a0)(?l)]
{3}[a0]{2}[(a0)(?d)]{3}[a0]{2}[(a0)(?!)]
{3}[a0]";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Full0,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Full0,
            syntax: FULL0,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FULL0,
                    [(
                        TokenKind::Plain,
                        r"{6}[(a0)(?H)]{3}[a0]{2}[(a0)(?e)]{3}[a0]
{2}[(a0)(?l)]{3}[a0]~{1}[a0]~{4}[(a0)(?o)]
{3}[a0]{2}[(a0)(?,)]{3}[a0]{2}[(a0)(? )]
{3}[a0]{2}[(a0)(?w)]{3}[a0]{2}[(a0)(?o)]
{3}[a0]{2}[(a0)(?r)]{3}[a0]{2}[(a0)(?l)]
{3}[a0]{2}[(a0)(?d)]{3}[a0]{2}[(a0)(?!)]
{3}[a0]",
                    )],
                )
            },
        }],
    };

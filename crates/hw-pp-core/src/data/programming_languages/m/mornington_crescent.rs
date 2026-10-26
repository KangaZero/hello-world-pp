//! Hello-world snippet for `ProgrammingLanguageName::MorningtonCrescent`.
//!
//! Imported from the Esolang wiki (`Mornington Crescent`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//! Written by Martin Ender.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Mornington_Crescent#Hello,_World!>

use crate::data::macros::language_names;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

language_names! {
    language: MorningtonCrescent => "Mornington Crescent",
    versions: [MorningtonCrescent => "Mornington Crescent"],
    extensions: [],
}

const MORNINGTON_CRESCENT: &str = r"Take Northern Line to Hendon Central
Take Northern Line to Bank
Take Northern Line to Bank
Take District Line to Gunnersbury
Take District Line to Victoria
Take Victoria Line to Seven Sisters
Take Victoria Line to Victoria
Take Victoria Line to Victoria
Take District Line to Bank
Take District Line to Hammersmith
Take District Line to Cannon Street
Take District Line to Hammersmith
Take District Line to Cannon Street
Take District Line to Bank
Take District Line to Hammersmith
Take District Line to Upminster
Take District Line to Hammersmith
Take District Line to Upminster
Take District Line to Gunnersbury
Take District Line to Paddington
Take District Line to Acton Town
Take Piccadilly Line to Holloway Road
Take Piccadilly Line to Acton Town
Take Piccadilly Line to Acton Town
Take District Line to Gunnersbury
Take District Line to Hammersmith
Take District Line to Notting Hill Gate
Take District Line to Upminster
Take District Line to Notting Hill Gate
Take District Line to Upminster
Take District Line to Victoria
Take Victoria Line to Seven Sisters
Take Victoria Line to Victoria
Take Victoria Line to Victoria
Take District Line to Upminster
Take District Line to Gunnersbury
Take District Line to Mile End
Take District Line to Hammersmith
Take District Line to Notting Hill Gate
Take District Line to Upminster
Take District Line to Upminster
Take District Line to Mile End
Take District Line to Paddington
Take District Line to Paddington
Take District Line to Acton Town
Take Piccadilly Line to Heathrow Terminals 1, 2, 3
Take Piccadilly Line to Holborn
Take Central Line to Holborn
Take Central Line to Mile End
Take District Line to Upminster
Take District Line to Hammersmith
Take District Line to Upminster
Take District Line to Barking
Take District Line to Hammersmith
Take District Line to Upminster
Take District Line to Gunnersbury
Take District Line to Barking
Take District Line to Gunnersbury
Take District Line to Paddington
Take District Line to Paddington
Take Circle Line to Wood Lane
Take Circle Line to Victoria
Take Circle Line to Victoria
Take District Line to Gunnersbury
Take District Line to Hammersmith
Take District Line to Upminster
Take District Line to Gunnersbury
Take District Line to Paddington
Take District Line to Paddington
Take District Line to Mile End
Take Central Line to Fairlop
Take Central Line to Mile End
Take District Line to Barking
Take District Line to Upminster
Take District Line to Upminster
Take District Line to Hammersmith
Take District Line to Notting Hill Gate
Take District Line to Upminster
Take District Line to Mile End
Take District Line to Gunnersbury
Take District Line to Paddington
Take District Line to Paddington
Take District Line to Hammersmith
Take District Line to Mile End
Take District Line to Richmond
Take District Line to Mile End
Take District Line to Paddington
Take District Line to Paddington
Take District Line to Richmond
Take District Line to Bank
Take District Line to Hammersmith
Take District Line to Upminster
Take District Line to Stepney Green
Take District Line to Hammersmith
Take District Line to Stepney Green
Take District Line to Upney
Take District Line to Notting Hill Gate
Take District Line to Notting Hill Gate
Take District Line to Notting Hill Gate
Take District Line to Upminster
Take District Line to Upney
Take District Line to Upminster
Take District Line to Bank
Take Circle Line to Bank
Take Northern Line to Charing Cross
Take Bakerloo Line to Charing Cross
Take Bakerloo Line to Paddington
Take Circle Line to Bank
Take Circle Line to Bank
Take Northern Line to Mornington Crescent";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MorningtonCrescent,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MorningtonCrescent,
            syntax: MORNINGTON_CRESCENT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MORNINGTON_CRESCENT,
                    [(TokenKind::Plain, MORNINGTON_CRESCENT)],
                )
            },
        }],
    };

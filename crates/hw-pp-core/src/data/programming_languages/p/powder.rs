//! Hello-world snippet for `ProgrammingLanguageName::Powder`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Powder>
//! - <https://esolangs.org/wiki/Powder>

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Structured,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Powder => "Powder",
    versions: [Powder => "Powder"],
    extensions: [],
}

const POWDER: &str = r#"{
	"Output" :
	{
		"1" : 72,
		"2" : 101,
		"3" : 108,
		"4" : 108,
		"5" : 111,
		"6" : 44,
		"7" : 32,
		"8" : 119,
		"9" : 111,
		"10" : 114,
		"11" : 108,
		"12" : 100
		"13" : 33
	}
}"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Powder,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, Structured, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Powder,
            syntax: POWDER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POWDER,
                    [(
                        TokenKind::Plain,
                        r#"{
	"Output" :
	{
		"1" : 72,
		"2" : 101,
		"3" : 108,
		"4" : 108,
		"5" : 111,
		"6" : 44,
		"7" : 32,
		"8" : 119,
		"9" : 111,
		"10" : 114,
		"11" : 108,
		"12" : 100
		"13" : 33
	}
}"#,
                    )],
                )
            },
        }],
    };

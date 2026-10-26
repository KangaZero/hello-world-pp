//! Hello-world snippet for `ProgrammingLanguageName::Binodu`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#binodu>
//! - <https://esolangs.org/wiki/Binodu>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Binodu => "binodu",
    versions: [Binodu => "binodu"],
    extensions: [],
}

const BINODU: &str = r" Automatic Node Hey
	Compare
	Action
		Output False
		Output True
		Fire TwoFalse
		Output True
		Output False
		Fire TwoFalse
		Fire Six
		Output False
		Output True
		Output False
		Output True
		Fire L
		Fire L
		Fire Six
		Fire TwoTrue
		Fire TwoTrue
		Fire Two
		Fire TwoTrue
		Fire TwoFalse
		Fire TwoFalse
		Fire TwoTrue
		Fire Two
		Fire Seven
		Fire Seven
		Fire Six
		Fire TwoTrue
		Fire TwoTrue
		Fire Seven
		Fire Two
		Fire L
		Fire Six
		Output False
		Output True
		Fire TwoFalse
		Fire Two
		Fire TwoFalse
		Fire False
		Fire True
		Stop
Node Six
	Compare
	Action
		Output False
		Fire TwoTrue
		Output False
Node L
	Compare
	Action
		Fire Six
		Fire TwoTrue
		Fire TwoFalse
Node Two
	Compare
	Action
		Fire TwoFalse
		Output True
		Output False
Node Seven
	Compare
	Action
		Output False
		Output True
		Fire TwoTrue
Node TwoTrue
	Compare
	Action
		Output True
		Output True
Node TwoFalse
	Compare
	Action
		Output False
		Output False";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Binodu,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Binodu,
            syntax: BINODU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BINODU,
                    [(
                        TokenKind::Plain,
                        r" Automatic Node Hey
	Compare
	Action
		Output False
		Output True
		Fire TwoFalse
		Output True
		Output False
		Fire TwoFalse
		Fire Six
		Output False
		Output True
		Output False
		Output True
		Fire L
		Fire L
		Fire Six
		Fire TwoTrue
		Fire TwoTrue
		Fire Two
		Fire TwoTrue
		Fire TwoFalse
		Fire TwoFalse
		Fire TwoTrue
		Fire Two
		Fire Seven
		Fire Seven
		Fire Six
		Fire TwoTrue
		Fire TwoTrue
		Fire Seven
		Fire Two
		Fire L
		Fire Six
		Output False
		Output True
		Fire TwoFalse
		Fire Two
		Fire TwoFalse
		Fire False
		Fire True
		Stop
Node Six
	Compare
	Action
		Output False
		Fire TwoTrue
		Output False
Node L
	Compare
	Action
		Fire Six
		Fire TwoTrue
		Fire TwoFalse
Node Two
	Compare
	Action
		Fire TwoFalse
		Output True
		Output False
Node Seven
	Compare
	Action
		Output False
		Output True
		Fire TwoTrue
Node TwoTrue
	Compare
	Action
		Output True
		Output True
Node TwoFalse
	Compare
	Action
		Output False
		Output False",
                    )],
                )
            },
        }],
    };

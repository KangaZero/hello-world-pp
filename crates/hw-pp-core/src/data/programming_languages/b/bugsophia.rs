//! Hello-world snippet for `ProgrammingLanguageName::Bugsophia`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#bugSophia>
//! - <https://esolangs.org/wiki/BugSophia>

use crate::data::programming_languages::ProgrammingParadigm::Concurrent;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Bugsophia => "bugSophia",
    versions: [Bugsophia => "bugSophia"],
    extensions: [],
}

const BUGSOPHIA: &str = r#" "+-+-+-+-+-+-+-+-+-+-+-+-+
  ` ` ` ` ` ` ` ` ` ` ` ` `
  H e l l o ,   W o r l d !
ᛥ'<-<-<-<-<-<-<-<-<-<-<-<-<"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bugsophia,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bugsophia,
            syntax: BUGSOPHIA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BUGSOPHIA,
                    [(
                        TokenKind::Plain,
                        r#" "+-+-+-+-+-+-+-+-+-+-+-+-+
  ` ` ` ` ` ` ` ` ` ` ` ` `
  H e l l o ,   W o r l d !
ᛥ'<-<-<-<-<-<-<-<-<-<-<-<-<"#,
                    )],
                )
            },
        }],
    };

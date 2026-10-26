//! Hello-world snippet for `ProgrammingLanguageName::U05eaU05dcU05ea`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#%D7%AA%D7%9C%D7%AA>
//! - <https://esolangs.org/wiki/Triliteral>

use crate::data::programming_languages::ProgrammingParadigm::{
    Concatenative, Imperative, Reflective,
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
    language: U05eaU05dcU05ea => "תלת",
    versions: [U05eaU05dcU05ea => "תלת"],
    extensions: [],
}

const U05EA_U05DC_U05EA: &str = r"אכטב  יטא  יכטב לאטר לוטאר ילטר סאכף וסאכאף אכלט ק יכלט אסוכף אסיכף יליטאר כאטאב אסמח א יסמח אכוטאב וסיכף עב קא קח קח קיא לב קיט קיא קיד קח ק לג י";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U05eaU05dcU05ea,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, Reflective, Concatenative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U05eaU05dcU05ea,
            syntax: U05EA_U05DC_U05EA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    U05EA_U05DC_U05EA,
                    [(
                        TokenKind::Plain,
                        r"אכטב  יטא  יכטב לאטר לוטאר ילטר סאכף וסאכאף אכלט ק יכלט אסוכף אסיכף יליטאר כאטאב אסמח א יסמח אכוטאב וסיכף עב קא קח קח קיא לב קיט קיא קיד קח ק לג י",
                    )],
                )
            },
        }],
    };

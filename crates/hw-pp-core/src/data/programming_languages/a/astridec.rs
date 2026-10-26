//! Hello-world snippet for `ProgrammingLanguageName::Astridec`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#Astridec>
//! - <https://esolangs.org/wiki/Astridec>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Astridec => "Astridec",
    versions: [Astridec => "Astridec"],
    extensions: [],
}

const ASTRIDEC: &str = r"11111111111111111111111111111111111111111111111111111111111111111111111183                *H*
11111111111111111111111111111111111111111111111111111111111111111111183                   *E*
111111111111111111111111111111111111111111111111111111111111111111111111111183            *L*
111111111111111111111111111111111111111111111111111111111111111111111111111183            *L*
111111111111111111111111111111111111111111111111111111111111111111111111111111183         *O*
1111111111111111111111111111111183                                                        * * 
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111183 *W*
111111111111111111111111111111111111111111111111111111111111111111111111111111183         *O*
111111111111111111111111111111111111111111111111111111111111111111111111111111111183      *R*
111111111111111111111111111111111111111111111111111111111111111111111111111183            *L*
1111111111111111111111111111111111111111111111111111111111111111111183                    *D*
*the newlines are not required*";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Astridec,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Astridec,
            syntax: ASTRIDEC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASTRIDEC,
                    [(
                        TokenKind::Plain,
                        r"11111111111111111111111111111111111111111111111111111111111111111111111183                *H*
11111111111111111111111111111111111111111111111111111111111111111111183                   *E*
111111111111111111111111111111111111111111111111111111111111111111111111111183            *L*
111111111111111111111111111111111111111111111111111111111111111111111111111183            *L*
111111111111111111111111111111111111111111111111111111111111111111111111111111183         *O*
1111111111111111111111111111111183                                                        * * 
11111111111111111111111111111111111111111111111111111111111111111111111111111111111111183 *W*
111111111111111111111111111111111111111111111111111111111111111111111111111111183         *O*
111111111111111111111111111111111111111111111111111111111111111111111111111111111183      *R*
111111111111111111111111111111111111111111111111111111111111111111111111111183            *L*
1111111111111111111111111111111111111111111111111111111111111111111183                    *D*
*the newlines are not required*",
                    )],
                )
            },
        }],
    };

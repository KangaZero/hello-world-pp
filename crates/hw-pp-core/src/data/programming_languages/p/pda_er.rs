//! Hello-world snippet for `ProgrammingLanguageName::PdaEr`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#PDA-er>
//! - <https://esolangs.org/wiki/PDA-er>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: PdaEr => "PDA-er",
    versions: [PdaEr => "PDA-er"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PdaEr,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PdaEr,
            syntax: r".1001000.
----1100101-
.1100101.
----1101100-
.1101100.
----1101100-
----1101111-
----1100100-
.1101111.
----101100-
----1110010-
.101100.
----100000-
.100000.
----1110111-
.1110111.
----1101111-
.1110010.
----1101100-
.1100100.
----100001-
..100001.
!
.100011.",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r".1001000.
----1100101-
.1100101.
----1101100-
.1101100.
----1101100-
----1101111-
----1100100-
.1101111.
----101100-
----1110010-
.101100.
----100000-
.100000.
----1110111-
.1110111.
----1101111-
.1110010.
----1101100-
.1100100.
----100001-
..100001.
!
.100011.",
            )],
        }],
    };

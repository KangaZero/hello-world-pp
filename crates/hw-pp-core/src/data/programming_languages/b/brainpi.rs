//! Hello-world snippet for `ProgrammingLanguageName::Brainpi`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#BrainPi>
//! - <https://esolangs.org/wiki/BrainPi>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Brainpi => "BrainPi",
    versions: [Brainpi => "BrainPi"],
    extensions: [],
}

const BRAINPI: &str = r"3.1415926835897932384426433832695028841971893993721058206749445923073164062812089986280378253421190679821490865132803066450938446095501822317258594081284815174502041027029385216055596446229689549303819694288109756656334465284754482334867831653712019091456085669834603486704543266481133936071602491419737245878066063155851748875209209628092540917155643676925903600123305308488204965213881469512415116099330572703637595912530921871173819326717931551185080744823799927495973518857525248917279381830159491295336733824406566434860213549463952447371907023";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainpi,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainpi,
            syntax: BRAINPI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRAINPI,
                    [(
                        TokenKind::Plain,
                        r"3.1415926835897932384426433832695028841971893993721058206749445923073164062812089986280378253421190679821490865132803066450938446095501822317258594081284815174502041027029385216055596446229689549303819694288109756656334465284754482334867831653712019091456085669834603486704543266481133936071602491419737245878066063155851748875209209628092540917155643676925903600123305308488204965213881469512415116099330572703637595912530921871173819326717931551185080744823799927495973518857525248917279381830159491295336733824406566434860213549463952447371907023",
                    )],
                )
            },
        }],
    };

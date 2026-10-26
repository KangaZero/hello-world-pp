//! Hello-world snippet for `ProgrammingLanguageName::GurYvsr`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#gur_yvsr>
//! - <https://esolangs.org/wiki/Gur_yvsr>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: GurYvsr => "gur yvsr",
    versions: [GurYvsr => "gur yvsr"],
    extensions: [],
}

const GUR_YVSR: &str = r"#72U   `H`
#101U  `e`
#108uU `ll`
#111U  `o`
#32U   ` `
#87U   `W`
#111U  `o`
#114U  `r`
#108U  `l`
#100U  `d`
#0k    `go to cell 0`
sssssssssss. `print out characters`";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GurYvsr,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GurYvsr,
            syntax: GUR_YVSR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GUR_YVSR,
                    [(
                        TokenKind::Plain,
                        r"#72U   `H`
#101U  `e`
#108uU `ll`
#111U  `o`
#32U   ` `
#87U   `W`
#111U  `o`
#114U  `r`
#108U  `l`
#100U  `d`
#0k    `go to cell 0`
sssssssssss. `print out characters`",
                    )],
                )
            },
        }],
    };

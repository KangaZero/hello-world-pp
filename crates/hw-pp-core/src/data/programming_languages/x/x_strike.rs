//! Hello-world snippet for `ProgrammingLanguageName::XStrike`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#X_strike>
//! - <https://esolangs.org/wiki/X_strike>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: XStrike => "X strike",
    versions: [XStrike => "X strike"],
    extensions: [],
}

const X_STRIKE: &str = r"X!×72
x^-1×0×-1
X!×29
x^-1×1×-1
X!×7
x^-1×2×-1
x^-1×2×-1
X!×3
x^-1×3×-1
x!×67
x^-1×4×-1
x!×12
x^-1×5×-1
X!×55
x^-1×6×-1
X!×24
x^-1×7×-1
X!×3
x^-1×8×-1
x!×6
x^-1×9×-1
x!×8
x^-1×10×-1
x!×67
x^-1×11×-1";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::XStrike,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::XStrike,
            syntax: X_STRIKE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    X_STRIKE,
                    [(
                        TokenKind::Plain,
                        r"X!×72
x^-1×0×-1
X!×29
x^-1×1×-1
X!×7
x^-1×2×-1
x^-1×2×-1
X!×3
x^-1×3×-1
x!×67
x^-1×4×-1
x!×12
x^-1×5×-1
X!×55
x^-1×6×-1
X!×24
x^-1×7×-1
X!×3
x^-1×8×-1
x!×6
x^-1×9×-1
x!×8
x^-1×10×-1
x!×67
x^-1×11×-1",
                    )],
                )
            },
        }],
    };

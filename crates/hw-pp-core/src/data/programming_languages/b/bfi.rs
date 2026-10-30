//! Hello-world snippet for `ProgrammingLanguageName::Bfi`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#BFI>
//! - <https://esolangs.org/wiki/BFI>

use crate::data::programming_languages::ProgrammingParadigm::{CellBased, StackBased};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Bfi => "BFI",
    versions: [Bfi => "BFI"],
    extensions: [],
}

const BFI: &str = r"++}}??{?>++.>+.+++}?..+++.>++.<<+++???.>.+++.------.--------.>+.>.;
>+++}?>++??{>+++>+<<<<-?;
++++ ";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bfi,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased, CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bfi,
            syntax: BFI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BFI,
                    [(
                        TokenKind::Plain,
                        r"++}}??{?>++.>+.+++}?..+++.>++.<<+++???.>.+++.------.--------.>+.>.;
>+++}?>++??{>+++>+<<<<-?;
++++ ",
                    )],
                )
            },
        }],
    };

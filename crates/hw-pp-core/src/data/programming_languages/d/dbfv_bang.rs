//! Hello-world snippet for `ProgrammingLanguageName::DbfvBang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#DBFV!>
//! - <https://esolangs.org/wiki/DBFV!>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: DbfvBang => "DBFV!",
    versions: [DbfvBang => "DBFV!"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::DbfvBang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::DbfvBang,
            syntax: r"Z<9Z+A<Z.YA:[A-*YZ].AB<7C<2~1A<1B<0C<1~1A<1B<0C<8~1A<1B<0C<8~1A<1B<1C<1~1.AB<4C<4~1.AB<3C<2~1A<1B<1C<9~1A<1B<1C<1~1A<1B<1C<4~1A<1B<0C<8~1A<1B<0C<0~1.AB<3C<3~1$0
D<CB:[B-*CZ]A:[A-*CY]C>^",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"Z<9Z+A<Z.YA:[A-*YZ].AB<7C<2~1A<1B<0C<1~1A<1B<0C<8~1A<1B<0C<8~1A<1B<1C<1~1.AB<4C<4~1.AB<3C<2~1A<1B<1C<9~1A<1B<1C<1~1A<1B<1C<4~1A<1B<0C<8~1A<1B<0C<0~1.AB<3C<3~1$0
D<CB:[B-*CZ]A:[A-*CY]C>^",
            )],
        }],
    };

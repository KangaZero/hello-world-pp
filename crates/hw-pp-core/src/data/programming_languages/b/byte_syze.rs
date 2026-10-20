//! Hello-world snippet for `ProgrammingLanguageName::ByteSyze`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Byte_Syze>
//! - <https://esolangs.org/wiki/Byte_Syze>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: ByteSyze => "Byte Syze",
    versions: [ByteSyze => "Byte Syze"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ByteSyze,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ByteSyze,
            syntax: r"000000: c7 3c 2a 3c 2a 2b 2a 5c  3c 29 5c 2a 2b 2a 5c 3c  .<*<*+*\<)\*+*\<
000010: 29 5c 2a 2b 2a 5c 3c 29  5c 2a 2b 2a 5c 3c 29 5c  )\*+*\<)\*+*\<)\
000020: 2a 2b 2a 5c 3c 29 5c 2a  2b 2a 5c 3c 29 5c 2a 2b  *+*\<)\*+*\<)\*+
000030: 2a 5c 3c 29 5c 2a 2b 2a  5c 3c 29 5c 2a 2b 2a 5c  *\<)\*+*\<)\*+*\
000040: 3c 29 5c 2a 2b 2a 5c 3c  29 5c 2a 2b 2a 5c 3c 29  <)\*+*\<)\*+*\<)
000050: 5c 2a 2b 2a 5c 3c 29 5c  2a 2b 2a 5c 3c 29 5c 2a  \*+*\<)\*+*\<)\*
000060: 2b 2a 00 00 01 00 00 00  00 00 00 00 00 00 00 00  +*..............
000070: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
000080: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
000090: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
0000a0: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
0000b0: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
0000c0: 00 00 00 00 00 00 00 64  48 65 6c 6c 6f 2c 20 57  .......dHello,.W
0000d0: 6f 72 6c 64 21 00 00 00  00 00 00 00 00 00 00 00  orld!...........",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"000000: c7 3c 2a 3c 2a 2b 2a 5c  3c 29 5c 2a 2b 2a 5c 3c  .<*<*+*\<)\*+*\<
000010: 29 5c 2a 2b 2a 5c 3c 29  5c 2a 2b 2a 5c 3c 29 5c  )\*+*\<)\*+*\<)\
000020: 2a 2b 2a 5c 3c 29 5c 2a  2b 2a 5c 3c 29 5c 2a 2b  *+*\<)\*+*\<)\*+
000030: 2a 5c 3c 29 5c 2a 2b 2a  5c 3c 29 5c 2a 2b 2a 5c  *\<)\*+*\<)\*+*\
000040: 3c 29 5c 2a 2b 2a 5c 3c  29 5c 2a 2b 2a 5c 3c 29  <)\*+*\<)\*+*\<)
000050: 5c 2a 2b 2a 5c 3c 29 5c  2a 2b 2a 5c 3c 29 5c 2a  \*+*\<)\*+*\<)\*
000060: 2b 2a 00 00 01 00 00 00  00 00 00 00 00 00 00 00  +*..............
000070: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
000080: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
000090: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
0000a0: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
0000b0: 00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
0000c0: 00 00 00 00 00 00 00 64  48 65 6c 6c 6f 2c 20 57  .......dHello,.W
0000d0: 6f 72 6c 64 21 00 00 00  00 00 00 00 00 00 00 00  orld!...........",
            )],
        }],
    };

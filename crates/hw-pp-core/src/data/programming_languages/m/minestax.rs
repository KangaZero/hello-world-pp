//! Hello-world snippet for `ProgrammingLanguageName::Minestax`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#MineStax>
//! - <https://esolangs.org/wiki/MineStax>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Minestax => "MineStax",
    versions: [Minestax => "MineStax"],
    extensions: [],
}

const MINESTAX: &str = r"liidsdsdsslliiiscliscliiscliidsiiiscliiislliidsdsdsslliidsdsdssl
liidsdsdsslliidsdsdsslliidsdsdsslliiiscliidsdsiiiscliislliiiscli
iscliidsscliiscliidsislliiiscliscliidsiscliiscliidsislliiisclisc
liidsiscliiscliidsislliiiscliiiscliscliiiscliidsiislliidsscliids
iscliidsiiislliidsdsdsslliidsdsdsslliiiscliscliidsdsiiiislliiisc
liscliidsdsiiiislliiiscliidsdsiiiscliislliiiscliidsdsiiiscliisll
iiiscliscliidsdsiscliscliislliiiscliscliidsdsiscliscliislliidsds
dsslliidsdsdsslliidsdsiscliscliidsiislliidsdsiscliiscliidsisllii
dsdsiscliiscliidsislliidsdsiscliscliidsiislliidsiiiscliiiscliids
iislliiiscliidsdsscliidsislliidsdsdsslliidsdsdsslliidsiiscliscli
idsdsislliidsscliidsiiscliidsiislliiiscliiscliiiscliiscliidsiisl
liiiscliscliidsiscliiscliidsislliiiscliscliidsiscliiscliidsislli
iiscliiscliiiscliiscliidsiislliidsscliidsiiscliidsiislliidsiiscl
iscliidsdsislliidsdsiiscliiiscliiislliiiscliidsdsiiscliiislliiis
cliidsscliidsdsislliidsiscliidsscliidsiiislliidsiiscliidsscliids
iislliidsscliidsscliidsdsslliiiscliidsiiscliidsiiislliiiscliidsd
siiscliiislliidsdsiiiiscliscliiislliidsdsdsslliidsdsdsslliidsdsd
sslliidsdsdsslliidsdsdsslliidsdsdsslliidsdsdsslliidsdsdsslliidsd
sdsslliidsscliidsiscliidsiiislliiiscliiiscliiscliiscliidsiisllii
iscliscliidsiscliiscliidsislliiiscliscliidsiscliiscliidsislliiis
cliiscliidsscliscliidsiislliidsscliidsiiscliidsiislliidsiscliiis
cliidsdsslliidsdsdsslliiiscliscliidsdsiiiislliiiscliscliidsdsiii
islliiiscliscliidsdsiiiislliiiscliidsdsiiiscliislliiiscliscliids
dsscliiscliislliiiscliscliidsdsiscliscliislliiiscliscliidsdsiiii
slliidsdsdsslliidsdsdsslliiiscliscliidsdsiiiislliiiscliscliidsds
iiiislliiiscliidsdsiiiscliislliiiscliidsdsiiiscliislliiiscliscli
idsdsiscliscliislliiiscliscliidsdsiscliscliislliidsdsdsslliidsds
dsslliiiscliscliiscliiiscliidsiiislliiiscliscliiscliidsscliidsii
slliiiscliscliiscliiscliscliiscliidsislliiiscliscliiscliiscliscl
iiscliidsislliiiscliiscliscliiscliscliiscliidsislliidsscliidsiis
cliidsiislliidsiscliidsscliidsiiislliidsdsdsslliiiscliidsdsiiscl
iiislliiiscliidsdsiiscliiislliidsiiiscliiscliidsiiislliidsiiiscl
iiscliidsiiislliidsiiiscliiscliidsiiislliidsiiiscliiscliidsiiisl
liiiscliidsdsiiscliiislliidsdsdss";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Minestax,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Minestax,
            syntax: MINESTAX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MINESTAX,
                    [(
                        TokenKind::Plain,
                        r"liidsdsdsslliiiscliscliiscliidsiiiscliiislliidsdsdsslliidsdsdssl
liidsdsdsslliidsdsdsslliidsdsdsslliiiscliidsdsiiiscliislliiiscli
iscliidsscliiscliidsislliiiscliscliidsiscliiscliidsislliiisclisc
liidsiscliiscliidsislliiiscliiiscliscliiiscliidsiislliidsscliids
iscliidsiiislliidsdsdsslliidsdsdsslliiiscliscliidsdsiiiislliiisc
liscliidsdsiiiislliiiscliidsdsiiiscliislliiiscliidsdsiiiscliisll
iiiscliscliidsdsiscliscliislliiiscliscliidsdsiscliscliislliidsds
dsslliidsdsdsslliidsdsiscliscliidsiislliidsdsiscliiscliidsisllii
dsdsiscliiscliidsislliidsdsiscliscliidsiislliidsiiiscliiiscliids
iislliiiscliidsdsscliidsislliidsdsdsslliidsdsdsslliidsiiscliscli
idsdsislliidsscliidsiiscliidsiislliiiscliiscliiiscliiscliidsiisl
liiiscliscliidsiscliiscliidsislliiiscliscliidsiscliiscliidsislli
iiscliiscliiiscliiscliidsiislliidsscliidsiiscliidsiislliidsiiscl
iscliidsdsislliidsdsiiscliiiscliiislliiiscliidsdsiiscliiislliiis
cliidsscliidsdsislliidsiscliidsscliidsiiislliidsiiscliidsscliids
iislliidsscliidsscliidsdsslliiiscliidsiiscliidsiiislliiiscliidsd
siiscliiislliidsdsiiiiscliscliiislliidsdsdsslliidsdsdsslliidsdsd
sslliidsdsdsslliidsdsdsslliidsdsdsslliidsdsdsslliidsdsdsslliidsd
sdsslliidsscliidsiscliidsiiislliiiscliiiscliiscliiscliidsiisllii
iscliscliidsiscliiscliidsislliiiscliscliidsiscliiscliidsislliiis
cliiscliidsscliscliidsiislliidsscliidsiiscliidsiislliidsiscliiis
cliidsdsslliidsdsdsslliiiscliscliidsdsiiiislliiiscliscliidsdsiii
islliiiscliscliidsdsiiiislliiiscliidsdsiiiscliislliiiscliscliids
dsscliiscliislliiiscliscliidsdsiscliscliislliiiscliscliidsdsiiii
slliidsdsdsslliidsdsdsslliiiscliscliidsdsiiiislliiiscliscliidsds
iiiislliiiscliidsdsiiiscliislliiiscliidsdsiiiscliislliiiscliscli
idsdsiscliscliislliiiscliscliidsdsiscliscliislliidsdsdsslliidsds
dsslliiiscliscliiscliiiscliidsiiislliiiscliscliiscliidsscliidsii
slliiiscliscliiscliiscliscliiscliidsislliiiscliscliiscliiscliscl
iiscliidsislliiiscliiscliscliiscliscliiscliidsislliidsscliidsiis
cliidsiislliidsiscliidsscliidsiiislliidsdsdsslliiiscliidsdsiiscl
iiislliiiscliidsdsiiscliiislliidsiiiscliiscliidsiiislliidsiiiscl
iiscliidsiiislliidsiiiscliiscliidsiiislliidsiiiscliiscliidsiiisl
liiiscliidsdsiiscliiislliidsdsdss",
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Ndball`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#NDBall>
//! - <https://esolangs.org/wiki/NDBall>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Ndball => "NDBall",
    versions: [Ndball => "NDBall"],
    extensions: [],
}

const NDBALL: &str = r"(0) >0
{0,1} >0
{0,2} +
{0,3} Y[72,>1,>0]
{0,3|1,1} <0
{0,1|1,1} <1
{0,4} >2
{0,4|2,1} <0
{0,3|2,1} p
{2,1} >2
{2,2} +
{2,3} Y[101,>3,>2]
{2,3|3,1} <2
{2,1|3,1} <3
{2,4} >4
{2,4|4,1} <2
{2,3|4,1} p
{4,1} >4
{4,2} +
{4,3} Y[108,>5,>4]
{4,3|5,1} <4
{4,1|5,1} <5
{4,4} >6
{4,4|6,1} <4
{4,3|6,1} p
{4,2|6,1} p
{6,1} >6
{6,2} +
{6,3} Y[111,>7,>6]
{6,3|7,1} <6
{6,1|7,1} <7
{6,4} >8
{6,4|8,1} <6
{6,3|8,1} p
{8,1} >8
{8,2} -
{8,3} Y[33,>8,>9]
{8,3|9,1} <8
{8,1|9,1} <9
{8,4} >10
{8,4|10,1} <8
{8,3|10,1} p
{10,1} >10
{10,2} +
{10,3} Y[119,>11,>10]
{10,3|11,1} <10
{10,1|11,1} <11
{10,4} >12
{10,4|12,1} <10
{10,3|12,1} p
{12,1} >12
{12,2} -
{12,3} Y[112,>12,>13]
{12,3|13,1} <12
{12,1|13,1} <13
{12,4} >14
{12,4|14,1} <12
{12,3|14,1} p
{14,1} >14
{14,2} +
{14,3} Y[114,>15,>14]
{14,3|15,1} <14
{14,1|15,1} <15
{14,4} >16
{14,4|16,1} <14
{14,3|16,1} p
{16,1} >16
{16,2} -
{16,3} Y[109,>16,>17]
{16,3|17,1} <16
{16,1|17,1} <17
{16,4} >18
{16,4|18,1} <16
{16,3|18,1} p
{18,1} >18
{18,2} -
{18,3} Y[101,>18,>19]
{18,3|19,1} <18
{18,1|19,1} <19
{18,4} >20
{18,4|20,1} <18
{18,3|20,1} p
{18,2|20,1} E";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ndball,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ndball,
            syntax: NDBALL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NDBALL,
                    [(
                        TokenKind::Plain,
                        r"(0) >0
{0,1} >0
{0,2} +
{0,3} Y[72,>1,>0]
{0,3|1,1} <0
{0,1|1,1} <1
{0,4} >2
{0,4|2,1} <0
{0,3|2,1} p
{2,1} >2
{2,2} +
{2,3} Y[101,>3,>2]
{2,3|3,1} <2
{2,1|3,1} <3
{2,4} >4
{2,4|4,1} <2
{2,3|4,1} p
{4,1} >4
{4,2} +
{4,3} Y[108,>5,>4]
{4,3|5,1} <4
{4,1|5,1} <5
{4,4} >6
{4,4|6,1} <4
{4,3|6,1} p
{4,2|6,1} p
{6,1} >6
{6,2} +
{6,3} Y[111,>7,>6]
{6,3|7,1} <6
{6,1|7,1} <7
{6,4} >8
{6,4|8,1} <6
{6,3|8,1} p
{8,1} >8
{8,2} -
{8,3} Y[33,>8,>9]
{8,3|9,1} <8
{8,1|9,1} <9
{8,4} >10
{8,4|10,1} <8
{8,3|10,1} p
{10,1} >10
{10,2} +
{10,3} Y[119,>11,>10]
{10,3|11,1} <10
{10,1|11,1} <11
{10,4} >12
{10,4|12,1} <10
{10,3|12,1} p
{12,1} >12
{12,2} -
{12,3} Y[112,>12,>13]
{12,3|13,1} <12
{12,1|13,1} <13
{12,4} >14
{12,4|14,1} <12
{12,3|14,1} p
{14,1} >14
{14,2} +
{14,3} Y[114,>15,>14]
{14,3|15,1} <14
{14,1|15,1} <15
{14,4} >16
{14,4|16,1} <14
{14,3|16,1} p
{16,1} >16
{16,2} -
{16,3} Y[109,>16,>17]
{16,3|17,1} <16
{16,1|17,1} <17
{16,4} >18
{16,4|18,1} <16
{16,3|18,1} p
{18,1} >18
{18,2} -
{18,3} Y[101,>18,>19]
{18,3|19,1} <18
{18,1|19,1} <19
{18,4} >20
{18,4|20,1} <18
{18,3|20,1} p
{18,2|20,1} E",
                    )],
                )
            },
        }],
    };

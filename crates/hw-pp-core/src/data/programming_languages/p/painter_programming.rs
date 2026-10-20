//! Hello-world snippet for `ProgrammingLanguageName::PainterProgramming`.
//!
//! Imported from leachim6/hello-world (`p/Painter Programming` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Painter%20Programming>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: PainterProgramming => "Painter Programming",
    versions: [PainterProgramming => "Painter Programming"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PainterProgramming,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PainterProgramming,
            syntax: r"H=11
W=22

R!F!F!F!F!
LLFF
RF!F!F
RFF
LL![F!]

RFF F!F!F!
RFF R!F!F!
LFF L!F!F!
LLFFFR![F!]
RFFFFF

R!F!F!F!F!
LF!F!
FF

L![F!]
LLFFFF
LF!F!
F

F!F!F!
L[F!]
LF!F!
LF!F!F!
FFFR[F]

L![F!]!
LF!F
LF!F!
LL[F]
LF!F
LF!F!F!F!
RFF

R![F!]
LF!F!
LF!F!F!F!
LF!
RRFFF

R![F!]
LFFF
L!FLF!RFLF!
RFF
R!F!
RF!
LFFFLF

LL![F!]
LF!F!
FF

L!F!F!F!F!
RF!F
R[F!]!
RF!",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"H=11
W=22

R!F!F!F!F!
LLFF
RF!F!F
RFF
LL![F!]

RFF F!F!F!
RFF R!F!F!
LFF L!F!F!
LLFFFR![F!]
RFFFFF

R!F!F!F!F!
LF!F!
FF

L![F!]
LLFFFF
LF!F!
F

F!F!F!
L[F!]
LF!F!
LF!F!F!
FFFR[F]

L![F!]!
LF!F
LF!F!
LL[F]
LF!F
LF!F!F!F!
RFF

R![F!]
LF!F!
LF!F!F!F!
LF!
RRFFF

R![F!]
LFFF
L!FLF!RFLF!
RFF
R!F!
RF!
LFFFLF

LL![F!]
LF!F!
FF

L!F!F!F!F!
RF!F
R[F!]!
RF!",
            )],
        }],
    };

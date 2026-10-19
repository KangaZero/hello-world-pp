//! Hello-world snippet for `ProgrammingLanguageName::Bitbitjump`.
//!
//! Imported from leachim6/hello-world (`b/BitBitJump.bbj` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BitBitJump.bbj>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Bitbitjump => "BitBitJump",
    versions: [Bitbitjump => "BitBitJump"],
    extensions: [Bbj => "bbj"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bitbitjump,
        extensions: &[FileExtension::Bbj],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bitbitjump,
            syntax: r"       Z0:0 Z1:0
start: .deref p X
       .testH X print -1
print: .out X
       .add p BASE p
       0 0 start
       p:H X:0
       H:72 101 108
       108 111 44
       32 87 111
       114 108 100
       33 10 -1
       .include lib.bbj",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"       Z0:0 Z1:0
start: .deref p X
       .testH X print -1
print: .out X
       .add p BASE p
       0 0 start
       p:H X:0
       H:72 101 108
       108 111 44
       32 87 111
       114 108 100
       33 10 -1
       .include lib.bbj",
            )],
        }],
    };

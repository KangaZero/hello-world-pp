//! Hello-world snippet for `ProgrammingLanguageName::LookBang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Look!>
//! - <https://esolangs.org/wiki/Look!>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: LookBang => "Look!",
    versions: [LookBang => "Look!"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LookBang,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LookBang,
            syntax: r"C D# C C D# D# D# D# B G D# D# C D# F# G# C C G D# C C G C C C G# C A A A A D C# G# C G# D# G# D G G C B A C# A D C# G G F G D D D F C C D# D# C D# C F F C C D# F G# G F A D F A F C C D# F D D D D D D F D D D D D D D D F G# G D# F G D# D# F",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"C D# C C D# D# D# D# B G D# D# C D# F# G# C C G D# C C G C C C G# C A A A A D C# G# C G# D# G# D G G C B A C# A D C# G G F G D D D F C C D# D# C D# C F F C C D# F G# G F A D F A F C C D# F D D D D D D F D D D D D D D D F G# G D# F G D# D# F",
            )],
        }],
    };

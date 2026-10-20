//! Hello-world snippet for `ProgrammingLanguageName::Indec`.
//!
//! Imported from leachim6/hello-world (`i/InDec.indec` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/InDec.indec>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Indec => "InDec",
    versions: [Indec => "InDec"],
    extensions: [Indec => "indec"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Indec,
        extensions: &[FileExtension::Indec],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Indec,
            syntax: r"add2   @ N { N ++   ++ }
add4  @ N { N add2  add2 } 
add8   @ N { N add4  add4 }
add16 @ N { N add8  add8 }
add32  @ N { N add16 add16 }
add64 @ N { N add32 add32 }

SPACE add32 
H   add64 add8
d,H add add16 add8 add4
e,d add ++
l,e add add4 add2 ++
o,l add add2 ++
W,H add add8 add4 add2 ++
r,o add add2 ++

H&,e&,l&,l&,o&,SPACE&,W&,o&,r&,l&,d& .",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"add2   @ N { N ++   ++ }
add4  @ N { N add2  add2 } 
add8   @ N { N add4  add4 }
add16 @ N { N add8  add8 }
add32  @ N { N add16 add16 }
add64 @ N { N add32 add32 }

SPACE add32 
H   add64 add8
d,H add add16 add8 add4
e,d add ++
l,e add add4 add2 ++
o,l add add2 ++
W,H add add8 add4 add2 ++
r,o add add2 ++

H&,e&,l&,l&,o&,SPACE&,W&,o&,r&,l&,d& .",
            )],
        }],
    };

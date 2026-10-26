//! Hello-world snippet for `ProgrammingLanguageName::Rhokell`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Rhokell>
//! - <https://esolangs.org/wiki/Rhokell>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rhokell => "Rhokell",
    versions: [Rhokell => "Rhokell"],
    extensions: [],
}

const RHOKELL: &str = r#"(main) = (print 
             (cons (byte (4) (8)) # "H"
             (cons (byte (6) (5)) # "e"
             (cons (byte (6) (C)) # "l"
             (cons (byte (6) (C)) # "l"
             (cons (byte (6) (F)) # "o"
             (cons (byte (2) (C)) # ","
             (cons (byte (2) (0)) # " "
             (cons (byte (7) (7)) # "w"
             (cons (byte (6) (F)) # "o"
             (cons (byte (7) (2)) # "r"
             (cons (byte (6) (C)) # "l"
             (cons (byte (6) (4)) # "d"
             (cons (byte (2) (1)) # "!"
             (cons (byte (0) (A)) # "\n"
                   (nil))))))))))))))));
(print (cons x y)) = (then (output x) (print y));
(then a b) = b;"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rhokell,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rhokell,
            syntax: RHOKELL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RHOKELL,
                    [(
                        TokenKind::Plain,
                        r#"(main) = (print 
             (cons (byte (4) (8)) # "H"
             (cons (byte (6) (5)) # "e"
             (cons (byte (6) (C)) # "l"
             (cons (byte (6) (C)) # "l"
             (cons (byte (6) (F)) # "o"
             (cons (byte (2) (C)) # ","
             (cons (byte (2) (0)) # " "
             (cons (byte (7) (7)) # "w"
             (cons (byte (6) (F)) # "o"
             (cons (byte (7) (2)) # "r"
             (cons (byte (6) (C)) # "l"
             (cons (byte (6) (4)) # "d"
             (cons (byte (2) (1)) # "!"
             (cons (byte (0) (A)) # "\n"
                   (nil))))))))))))))));
(print (cons x y)) = (then (output x) (print y));
(then a b) = b;"#,
                    )],
                )
            },
        }],
    };

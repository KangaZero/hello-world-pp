//! Hello-world snippet for `ProgrammingLanguageName::Flipjump`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#FlipJump>
//! - <https://esolangs.org/wiki/FlipJump>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Flipjump => "FlipJump",
    versions: [Flipjump => "FlipJump"],
    extensions: [],
}

const FLIPJUMP: &str = concat!(
    r#"       ;start
IO:    ;
start:
       output ""#,
    output_for_hello_world!(),
    r#"\n"
end:   ;end

def output_bit bit < IO {
    bit ? IO+1 : IO;  // output (0/1) is done by flipping the 2w / 2w+1 bits.
}

def output str {
    rep(((#str)+7)&(0-8), i) output_bit (str>>i)&1  // # is the number of bits needed to represent the number. calls strbitlen() times to output_bit.
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flipjump,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flipjump,
            syntax: FLIPJUMP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FLIPJUMP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"       ;start
IO:    ;
start:
       output ""#,
                            output_for_hello_world!(),
                            r#"\n"
end:   ;end

def output_bit bit < IO {
    bit ? IO+1 : IO;  // output (0/1) is done by flipping the 2w / 2w+1 bits.
}

def output str {
    rep(((#str)+7)&(0-8), i) output_bit (str>>i)&1  // # is the number of bits needed to represent the number. calls strbitlen() times to output_bit.
}"#
                        ),
                    )],
                )
            },
        }],
    };

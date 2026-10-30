//! Hello-world snippet for `ProgrammingLanguageName::Slam`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Slam>
//! - <https://esolangs.org/wiki/Slam>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Slam => "Slam",
    versions: [Slam => "Slam"],
    extensions: [],
}

const SLAM: &str = concat!(
    r"Hi hi hi hi hi
Give me the key
Bye bye bye
Hi hi hi hi hi
The big world
Good morning
Bye bye
Hi hi hi hi hi
Hi good morning
Good morning
Bye bye bye
Hi hi hi hi
Bye bye
Bye bye bye
Bye bye bye
Hi hi hi hi hi
Hi hi bye bye bye
Bye bye bye
Bye bye bye
Good morning
Look
Hi hi hi hi hi
Hello HelloWorld!
Good morning
",
    output_for_hello_world!(),
    r"
Hi hi hi hi
Bye bye bye
",
    output_for_hello_world!(),
    r"
Bye bye bye
Good morning
Good morning
Hi hi hi hi hi
Hi hi HelloWorld!
Bye bye bye
Hi hi hi hi hi
Hi hi hi hi hi
Bye bye bye
Hi hi hi hi hi
HelloWorldProgram Morning
Bye bye bye
",
    output_for_hello_world!(),
    r"
Good morning
Bye bye bye
Hi hi hi hi hi
Hello! HelloWorldProgram!!
Bye bye bye
",
    output_for_hello_world!(),
    r"
Look
Bye bye bye
Hi hi hi hi hi
Hi hi ",
    output_for_hello_world!(),
    r"
Bye bye bye
Hi hi hi hi hi
Bye HelloWorld!
Bye bye bye
Hi hi hi hi hi
Hi again
Bye bye bye"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Slam,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Slam,
            syntax: SLAM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SLAM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"Hi hi hi hi hi
Give me the key
Bye bye bye
Hi hi hi hi hi
The big world
Good morning
Bye bye
Hi hi hi hi hi
Hi good morning
Good morning
Bye bye bye
Hi hi hi hi
Bye bye
Bye bye bye
Bye bye bye
Hi hi hi hi hi
Hi hi bye bye bye
Bye bye bye
Bye bye bye
Good morning
Look
Hi hi hi hi hi
Hello HelloWorld!
Good morning
",
                            output_for_hello_world!(),
                            r"
Hi hi hi hi
Bye bye bye
",
                            output_for_hello_world!(),
                            r"
Bye bye bye
Good morning
Good morning
Hi hi hi hi hi
Hi hi HelloWorld!
Bye bye bye
Hi hi hi hi hi
Hi hi hi hi hi
Bye bye bye
Hi hi hi hi hi
HelloWorldProgram Morning
Bye bye bye
",
                            output_for_hello_world!(),
                            r"
Good morning
Bye bye bye
Hi hi hi hi hi
Hello! HelloWorldProgram!!
Bye bye bye
",
                            output_for_hello_world!(),
                            r"
Look
Bye bye bye
Hi hi hi hi hi
Hi hi ",
                            output_for_hello_world!(),
                            r"
Bye bye bye
Hi hi hi hi hi
Bye HelloWorld!
Bye bye bye
Hi hi hi hi hi
Hi again
Bye bye bye"
                        ),
                    )],
                )
            },
        }],
    };

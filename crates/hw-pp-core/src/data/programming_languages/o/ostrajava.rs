//! Hello-world snippet for `ProgrammingLanguageName::Ostrajava`.
//!
//! Imported from leachim6/hello-world (`o/OSTRAJava.cyp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/OSTRAJava.cyp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ostrajava => "OSTRAJava",
    versions: [Ostrajava => "OSTRAJava"],
    extensions: [Cyp => "cyp"],
}

const OSTRAJAVA: &str = concat!(
    r#"banik pyco

tryda Ostrava{
    rynek(){
        Konzola.pravit(""#,
    output_for_hello_world!(),
    r#"") pyco
    }
} 

fajront pyco"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ostrajava,
        extensions: &[FileExtension::Cyp],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ostrajava,
            syntax: OSTRAJAVA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    OSTRAJAVA,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"banik pyco

tryda Ostrava{
    rynek(){
        Konzola.pravit(""#,
                            output_for_hello_world!(),
                            r#"") pyco
    }
} 

fajront pyco"#
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Orc`.
//!
//! Imported from leachim6/hello-world (`o/Orc.orc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/o/Orc.orc>
//! - <https://www.wikidata.org/wiki/Q7100142> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Concurrent;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Orc => "Orc",
    versions: [Orc => "Orc"],
    extensions: [Orc => "orc"],
}

const ORC: &str = concat!(
    r#"{- HelloWorld.orc -- Orc program HelloWorld
 -
 - $Id$
 -
 - Created by xbony2 on Nov 8, 2014 8:30:25 PM
 - Licensed under public domain.
 -}

Println(""#,
    output_for_hello_world!(),
    r#"") >>
stop"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Orc,
        extensions: &[FileExtension::Orc],
        categories: &[],
        families: &[],
        paradigms: &[Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Orc,
            syntax: ORC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ORC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"{- HelloWorld.orc -- Orc program HelloWorld
 -
 - $Id$
 -
 - Created by xbony2 on Nov 8, 2014 8:30:25 PM
 - Licensed under public domain.
 -}

Println(""#,
                            output_for_hello_world!(),
                            r#"") >>
stop"#
                        ),
                    )],
                )
            },
        }],
    };

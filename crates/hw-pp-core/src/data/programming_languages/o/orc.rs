//! Hello-world snippet for `ProgrammingLanguageName::Orc`.
//!
//! Imported from leachim6/hello-world (`o/Orc.orc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Orc => "Orc",
    versions: [Orc => "Orc"],
    extensions: [Orc => "orc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Orc,
        extensions: &[FileExtension::Orc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Orc,
            syntax: concat!(
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
            tokenized_syntax: &[(
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
        }],
    };

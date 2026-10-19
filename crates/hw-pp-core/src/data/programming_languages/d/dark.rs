//! Hello-world snippet for `ProgrammingLanguageName::Dark`.
//!
//! Imported from leachim6/hello-world (`d/dark.txt` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dark => "dark",
    versions: [Dark => "dark"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dark,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dark,
            syntax: concat!(
                r#"+helloworld hell
helloworld$twist stalker io
helloworld$twist sign string
io$stalk
io$personal
string$scrawl " "#,
                output_for_hello_world!(),
                r"
string$read ~
io$echo
helloworld$consume io
helloworld$consume string
helloworld$empty
helloworld$apocalypse"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"+helloworld hell
helloworld$twist stalker io
helloworld$twist sign string
io$stalk
io$personal
string$scrawl " "#,
                    output_for_hello_world!(),
                    r"
string$read ~
io$echo
helloworld$consume io
helloworld$consume string
helloworld$empty
helloworld$apocalypse"
                ),
            )],
        }],
    };

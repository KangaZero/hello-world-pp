//! Hello-world snippet for `ProgrammingLanguageName::Bibtex`.
//!
//! Imported from the Hello World Collection (`files/BibTex.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/BibTex.htm>
//! - <http://helloworldcollection.de/#BibTex>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Bibtex => "BibTex",
    versions: [Bibtex => "BibTex"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bibtex,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bibtex,
            syntax: concat!(
                r"%",
                output_for_hello_world!(),
                r#" in BibTex

ENTRY{author}{}{}

FUNCTION {hello.world}
{ 
  ""#,
                output_for_hello_world!(),
                r#"" write$ newline$
}

READ 
EXECUTE {hello.world}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"%",
                    output_for_hello_world!(),
                    r#" in BibTex

ENTRY{author}{}{}

FUNCTION {hello.world}
{ 
  ""#,
                    output_for_hello_world!(),
                    r#"" write$ newline$
}

READ 
EXECUTE {hello.world}"#
                ),
            )],
        }],
    };

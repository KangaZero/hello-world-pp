//! Hello-world snippet for `ProgrammingLanguageName::Texinfo`.
//!
//! Imported from the Hello World Collection (`files/Texinfo` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Texinfo>
//! - <http://helloworldcollection.de/#Texinfo>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Texinfo => "Texinfo",
    versions: [Texinfo => "Texinfo"],
    extensions: [],
}

const TEXINFO: &str = concat!(
    r"\input texinfo
@c ",
    output_for_hello_world!(),
    r" for Texinfo

@setfilename hello
@settitle ",
    output_for_hello_world!(),
    r"

@node Top, Hello, (dir), (dir)

@menu
* Hello:: ",
    output_for_hello_world!(),
    r"
@end menu

@node Hello, , Top, Top

",
    output_for_hello_world!(),
    r"

@contents
@bye"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Texinfo,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Texinfo,
            syntax: TEXINFO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TEXINFO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"\input texinfo
@c ",
                            output_for_hello_world!(),
                            r" for Texinfo

@setfilename hello
@settitle ",
                            output_for_hello_world!(),
                            r"

@node Top, Hello, (dir), (dir)

@menu
* Hello:: ",
                            output_for_hello_world!(),
                            r"
@end menu

@node Hello, , Top, Top

",
                            output_for_hello_world!(),
                            r"

@contents
@bye"
                        ),
                    )],
                )
            },
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Cweb`.
//!
//! Imported from the Hello World Collection (`files/CWEB.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/CWEB.htm>
//! - <http://helloworldcollection.de/#CWEB>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cweb => "CWEB",
    versions: [Cweb => "CWEB"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cweb,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cweb,
            syntax: concat!(
                r#"\def\title{<a href="/showArticle.jhtml?documentID=cuj9505wittenbe&pgno=1">Listing 1</a>}

@*A Simple Example.
This is a trivial example of a \.{CWEB} program.
It is, of course, the classic ""#,
                output_for_hello_world!(),
                r#""
program we all know and love:

@c
@<Header files needed by the program@>@;
@#
main(void)
{
   @<Print the message |""#,
                output_for_hello_world!(),
                r#""|@>@;
}

@ Naturally, we use |printf| to do the dirty work:

@<Print the message |""#,
                output_for_hello_world!(),
                r#""|@>=
printf(""#,
                output_for_hello_world!(),
                r#"\n");

@ The prototype for |printf| is in the standard
header, \.{<stdio.h>}.

@<Header files needed by the program@>=
#include <stdio.h>

@*Index."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"\def\title{<a href="/showArticle.jhtml?documentID=cuj9505wittenbe&pgno=1">Listing 1</a>}

@*A Simple Example.
This is a trivial example of a \.{CWEB} program.
It is, of course, the classic ""#,
                    output_for_hello_world!(),
                    r#""
program we all know and love:

@c
@<Header files needed by the program@>@;
@#
main(void)
{
   @<Print the message |""#,
                    output_for_hello_world!(),
                    r#""|@>@;
}

@ Naturally, we use |printf| to do the dirty work:

@<Print the message |""#,
                    output_for_hello_world!(),
                    r#""|@>=
printf(""#,
                    output_for_hello_world!(),
                    r#"\n");

@ The prototype for |printf| is in the standard
header, \.{<stdio.h>}.

@<Header files needed by the program@>=
#include <stdio.h>

@*Index."#
                ),
            )],
        }],
    };

//! Hello-world snippet for `ProgrammingLanguageName::Amharic`.
//!
//! Imported from the Hello World Collection (`files/amharic.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/amharic.txt>
//! - <http://helloworldcollection.de/#amharic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Amharic => "amharic",
    versions: [Amharic => "amharic"],
    extensions: [],
}

const AMHARIC: &str = concat!(
    output_for_hello_world!(),
    r#" in amharic (in JavaScript)

 
<%@ language="javascript" %>
<html><body>
<%
Response.Write('"#,
    output_for_hello_world!(),
    r"');
%>
</body></html>"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Amharic,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Amharic,
            syntax: AMHARIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AMHARIC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            output_for_hello_world!(),
                            r#" in amharic (in JavaScript)

 
<%@ language="javascript" %>
<html><body>
<%
Response.Write('"#,
                            output_for_hello_world!(),
                            r"');
%>
</body></html>"
                        ),
                    )],
                )
            },
        }],
    };

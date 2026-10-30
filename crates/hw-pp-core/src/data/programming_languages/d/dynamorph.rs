//! Hello-world snippet for `ProgrammingLanguageName::Dynamorph`.
//!
//! Imported from the Hello World Collection (`files/DynaMorph.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/DynaMorph.txt>
//! - <http://helloworldcollection.de/#DynaMorph>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dynamorph => "DynaMorph",
    versions: [Dynamorph => "DynaMorph"],
    extensions: [],
}

const DYNAMORPH: &str = concat!(
    r"<<!! ",
    output_for_hello_world!(),
    r" in DynaMorph >>
<#setString foo {",
    output_for_hello_world!(),
    r"}#>
<html>
<head>
	<title>DynaMorph</title>
</head>
<body>
<#getString foo#>
</body>
</html>"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dynamorph,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dynamorph,
            syntax: DYNAMORPH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DYNAMORPH,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"<<!! ",
                            output_for_hello_world!(),
                            r" in DynaMorph >>
<#setString foo {",
                            output_for_hello_world!(),
                            r"}#>
<html>
<head>
	<title>DynaMorph</title>
</head>
<body>
<#getString foo#>
</body>
</html>"
                        ),
                    )],
                )
            },
        }],
    };

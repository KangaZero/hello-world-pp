//! Hello-world snippet for `ProgrammingLanguageName::Parser`.
//!
//! Imported from leachim6/hello-world (`p/Parser.p` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Parser.p>
//! - <https://www.wikidata.org/wiki/Q3501043> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Parser => "Parser",
    versions: [Parser => "Parser"],
    extensions: [],
}

const PARSER: &str = concat!(
    r#"@main[]
  ^rem{Will print ""#,
    output_for_hello_world!(),
    r#"" when run as CGI script}
  $hello["#,
    output_for_hello_world!(),
    r"]
  $result[$hello]"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Parser,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Parser,
            syntax: PARSER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PARSER,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"@main[]
  ^rem{Will print ""#,
                            output_for_hello_world!(),
                            r#"" when run as CGI script}
  $hello["#,
                            output_for_hello_world!(),
                            r"]
  $result[$hello]"
                        ),
                    )],
                )
            },
        }],
    };

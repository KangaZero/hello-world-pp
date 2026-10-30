//! Hello-world snippet for `ProgrammingLanguageName::Emerald`.
//!
//! Imported from leachim6/hello-world (`e/Emerald.m` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Emerald.m>
//! - <https://www.wikidata.org/wiki/Q5370400> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Emerald => "Emerald",
    versions: [Emerald => "Emerald"],
    extensions: [],
}

const EMERALD: &str = concat!(
    r#"const main <- object mainProgram
    initially
		stdout.putString[""#,
    output_for_hello_world!(),
    r#"\n"]
	end initially
end mainProgram"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emerald,
        extensions: &[],
        categories: &[Compiled, Historical],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emerald,
            syntax: EMERALD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EMERALD,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"const main <- object mainProgram
    initially
		stdout.putString[""#,
                            output_for_hello_world!(),
                            r#"\n"]
	end initially
end mainProgram"#
                        ),
                    )],
                )
            },
        }],
    };

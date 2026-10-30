//! Hello-world snippet for `ProgrammingLanguageName::Lisaac`.
//!
//! Imported from leachim6/hello-world (`l/Lisaac.li` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/Lisaac.li>
//! - <https://www.wikidata.org/wiki/Q3242457> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Lisaac => "Lisaac",
    versions: [Lisaac => "Lisaac"],
    extensions: [],
}

const LISAAC: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in Lisaac

Section Header
	+ name		:= HELLO_WORLD;

Section Inherit

	- parent_object:OBJECT := OBJECT;

Section Public

	- main <-
	(
		""#,
    output_for_hello_world!(),
    r#" !\n".print;
	);"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lisaac,
        extensions: &[],
        categories: &[Compiled],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lisaac,
            syntax: LISAAC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LISAAC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in Lisaac

Section Header
	+ name		:= HELLO_WORLD;

Section Inherit

	- parent_object:OBJECT := OBJECT;

Section Public

	- main <-
	(
		""#,
                            output_for_hello_world!(),
                            r#" !\n".print;
	);"#
                        ),
                    )],
                )
            },
        }],
    };

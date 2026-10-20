//! Hello-world snippet for `ProgrammingLanguageName::Y`.
//!
//! Imported from leachim6/hello-world (`y/Y.y` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/y/Y.y>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Y => "Y",
    versions: [Y => "Y"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Y,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Y,
            syntax: concat!(
                r#"module helloworld

	define(EOF,(-1))
	
	import printf from "ylib.d"

	main()
		printf(""#,
                output_for_hello_world!(),
                r#"\n")
	end
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module helloworld

	define(EOF,(-1))
	
	import printf from "ylib.d"

	main()
		printf(""#,
                    output_for_hello_world!(),
                    r#"\n")
	end
end"#
                ),
            )],
        }],
    };

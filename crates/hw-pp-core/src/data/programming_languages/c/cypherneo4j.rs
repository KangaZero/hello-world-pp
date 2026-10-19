//! Hello-world snippet for `ProgrammingLanguageName::Cypherneo4j`.
//!
//! Imported from leachim6/hello-world (`c/CypherNeo4j.cypher` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cypherneo4j => "CypherNeo4j",
    versions: [Cypherneo4j => "CypherNeo4j"],
    extensions: [Cypher => "cypher"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cypherneo4j,
        extensions: &[FileExtension::Cypher],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cypherneo4j,
            syntax: concat!(
                r"CREATE (Hello:Word { val: 'Hello' }), (World:Word { val: 'World' }),
(Hello)-[:SPACE]->(World)
RETURN ",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"CREATE (Hello:Word { val: 'Hello' }), (World:Word { val: 'World' }),
(Hello)-[:SPACE]->(World)
RETURN ",
                    output_for_hello_world!()
                ),
            )],
        }],
    };

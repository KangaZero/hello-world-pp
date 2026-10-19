//! Hello-world snippet for `ProgrammingLanguageName::ReactVr`.
//!
//! Imported from the Hello World Collection (`files/React-VR.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/React-VR.htm>
//! - <http://helloworldcollection.de/#React-VR>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: ReactVr => "React-VR",
    versions: [ReactVr => "React-VR"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ReactVr,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ReactVr,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r" in React-VR

import React from 'react';
import { AppRegistry, asset, Text, View,} from 'react-vr';
 
export default class HelloWorld extends React.Component {
  render() { return (
      <View>
        <Text
          style={{transform: [{translate: [0, 0, -3]}],}}>
          ",
                output_for_hello_world!(),
                r"
        </Text>
      </View>
    );   } };
AppRegistry.registerComponent('HelloWorld', () => HelloWorld);"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r" in React-VR

import React from 'react';
import { AppRegistry, asset, Text, View,} from 'react-vr';
 
export default class HelloWorld extends React.Component {
  render() { return (
      <View>
        <Text
          style={{transform: [{translate: [0, 0, -3]}],}}>
          ",
                    output_for_hello_world!(),
                    r"
        </Text>
      </View>
    );   } };
AppRegistry.registerComponent('HelloWorld', () => HelloWorld);"
                ),
            )],
        }],
    };

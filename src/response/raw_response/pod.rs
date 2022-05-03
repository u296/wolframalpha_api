use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawPod {
    title: String,
    scanner: String,
    id: String,
    position: i32,
    error: bool,
    numsubpods: i32,
    subpods: Vec<RawSubPod>,
    expressiontypes: RawExpressionTypesWrapper,
    states: Option<Vec<RawStateWrapper>>,
    infos: Option<RawInfo>,
    primary: Option<bool>,
    definitions: Option<RawDefinitionsWrapper>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawSubPod {
    title: String,
    primary: Option<bool>,
    imagesource: Option<String>,
    microsources: Option<RawMicroSource>,
    datasources: Option<RawDataSource>,
    img: RawImage,
    plaintext: String,
    infos: Option<RawInfo>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawImage {
    pub src: String,
    pub alt: String,
    pub title: String,
    pub width: I32OrString,
    pub height: I32OrString,
    pub r#type: Option<String>,
    pub themes: Option<String>,
    pub colorinvertable: Option<bool>,
    pub contenttype: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum I32OrString {
    Num(i32),
    Text(String),
}

impl I32OrString {
    pub fn get_val(&self) -> i32 {
        match self {
            Self::Num(n) => *n,
            Self::Text(t) => t.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RawExpressionTypesWrapper {
    Single(RawExpressionType),
    Multi(Vec<RawExpressionType>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawExpressionType {
    name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RawMicroSourceWrapper {
    Single(String),
    Multi(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawMicroSource {
    microsource: RawMicroSourceWrapper,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RawDataSourceWrapper {
    Single(String),
    Multi(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawDataSource {
    datasource: RawDataSourceWrapper,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum RawStateWrapper {
    Single(RawState),
    Multi(RawMultiState),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawState {
    pub name: String,
    pub input: String,
    pub stepbystep: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawMultiState {
    pub count: i32,
    pub value: String,
    pub delimiters: String,
    pub states: Vec<RawState>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawInfo {
    pub units: Option<Vec<RawUnitsWrapper>>,
    pub text: Option<String>,
    pub img: Option<RawImage>,
    pub links: Option<RawLinksWrapper>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum RawLinksWrapper {
    Single(RawLink),
    Multi(Vec<RawLink>),
}

impl IntoIterator for RawLinksWrapper {
    type Item = RawLink;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Single(r) => vec![r].into_iter(),
            Self::Multi(v) => v.into_iter(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawLink {
    pub url: String,
    pub text: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum RawUnitsWrapper {
    MeasurementUnits(RawMeasurementUnitsWrapper),
    Src(RawUnitSource),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum RawMeasurementUnitsWrapper {
    Single(RawMeasurementUnit),
    Multi(Vec<RawMeasurementUnit>),
}

impl IntoIterator for RawMeasurementUnitsWrapper {
    type Item = RawMeasurementUnit;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Multi(v) => v.into_iter(),
            Self::Single(i) => vec![i].into_iter(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawMeasurementUnit {
    pub 
    short: String,
    pub 
    long: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawUnitSource {
    pub src: String,
    pub width: String,
    pub height: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawDefinition {
    word: String,
    desc: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum RawDefinitionsWrapper {
    Single(RawDefinition),
    Multi(Vec<RawDefinition>),
}

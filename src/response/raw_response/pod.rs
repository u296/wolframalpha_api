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
    src: String,
    alt: String,
    title: String,
    width: I32OrString,
    height: I32OrString,
    r#type: Option<String>,
    themes: Option<String>,
    colorinvertable: Option<bool>,
    contenttype: Option<String>,
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
    name: String,
    input: String,
    stepbystep: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawMultiState {
    count: i32,
    value: String,
    delimiters: String,
    states: Vec<RawState>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawInfo {
    units: Option<Vec<RawUnitsWrapper>>,
    text: Option<String>,
    img: Option<RawImage>,
    links: Option<RawLinksWrapper>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum RawLinksWrapper {
    Single(RawLink),
    Multi(Vec<RawLink>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawLink {
    url: String,
    text: String,
    title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum RawUnitsWrapper {
    TimeUnit(RawTimeUnitWrapper),
    Src(RawUnitSource),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum RawTimeUnitWrapper {
    Single(RawTimeUnit),
    Multi(Vec<RawTimeUnit>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawTimeUnit {
    short: String,
    long: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawUnitSource {
    src: String,
    width: String,
    height: String,
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

use serde::Deserialize;
use std::convert;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pod {
    title: String,
    scanner: String,
    id: String,
    position: i32,
    error: bool,
    numsubpods: i32,
    subpods: Vec<SubPod>,
    expressiontypes: ExpressionTypesWrapper,
    states: Option<Vec<StateWrapper>>,
    infos: Option<Info>,
    primary: Option<bool>,
    definitions: Option<DefinitionsWrapper>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubPod {
    title: String,
    primary: Option<bool>,
    imagesource: Option<String>,
    microsources: Option<MicroSource>,
    datasources: Option<DataSource>,
    img: Image,
    plaintext: String,
    infos: Option<Info>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Image {
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
pub enum ExpressionTypesWrapper {
    Single(ExpressionType),
    Multi(Vec<ExpressionType>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpressionType {
    name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MicroSourceWrapper {
    Single(String),
    Multi(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MicroSource {
    microsource: MicroSourceWrapper,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DataSourceWrapper {
    Single(String),
    Multi(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataSource {
    datasource: DataSourceWrapper,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum StateWrapper {
    Single(State),
    Multi(MultiState),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    name: String,
    input: String,
    stepbystep: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiState {
    count: i32,
    value: String,
    delimiters: String,
    states: Vec<State>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Info {
    units: Option<Vec<UnitsWrapper>>,
    text: Option<String>,
    img: Option<Image>,
    links: Option<LinksWrapper>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum LinksWrapper {
    Single(Link),
    Multi(Vec<Link>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    url: String,
    text: String,
    title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum UnitsWrapper {
    TimeUnit(TimeUnitWrapper),
    Src(UnitSource),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum TimeUnitWrapper {
    Single(TimeUnit),
    Multi(Vec<TimeUnit>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimeUnit {
    short: String,
    long: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnitSource {
    src: String,
    width: String,
    height: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Definition {
    word: String,
    desc: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum DefinitionsWrapper {
    Single(Definition),
    Multi(Vec<Definition>),
}

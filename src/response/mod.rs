pub mod raw_response;
use raw_response::{pod::*, *};

pub struct QueryResult {
    success: bool,
}

pub struct Pod {
    title: String,
    scanner: String,
    id: String,
    position: i32,
    subpods: Vec<SubPod>,
    expressiontypes: Vec<String>,
    states: Vec<State>,
    infos: Vec<Info>,
    primary: bool,
    definitions: Vec<Definition>,
}

struct SubPod {
    title: String,
    primary: bool,
    image_source: Option<String>,
    micro_sources: Vec<String>,
    data_sources: Vec<String>,
    img: Image,
    plaintext: String,
    infos: Vec<Info>,
}

pub struct Definition {
    word: String,
    description: String,
}

pub struct Info {
    units: Option<Units>,
    text: Option<String>,
    image: Option<Image>,
    links: Vec<Link>,
}

impl From<RawInfo> for Info {
    fn from(x: RawInfo) -> Self {
        Info {
            units: {
                match x.units {
                    Some(units) => {
                        let mut source: Option<UnitSource> = None;
                        let mut units_res: Vec<MeasurementUnit> = vec![];
                        for i in units.iter() {
                            match i {
                                RawUnitsWrapper::MeasurementUnits(u) => {units_res.extend(u.into_iter().map(Into::into));},
                                RawUnitsWrapper::Src(src) => {source = Some((*src).into());}
                            };
                        }

                        Some(Units{
                            units: units_res,
                            source: source.unwrap()
                        })
                        
                    },
                    None => None
                }
            },
            text: x.text,
            image: x.img.map(Into::into),
            links: {
                match x.links {
                    Some(links) => {
                        links.into_iter().map(Into::into).collect()
                    }, None => vec![]
                }
            }

        }
    }
}

struct Units {
    units: Vec<MeasurementUnit>,
    source: UnitSource,
}

enum Unit {
    Time(MeasurementUnit),
    Source(UnitSource),
}


struct MeasurementUnit {
    short: String,
    long: String,
}

impl From<RawMeasurementUnit> for MeasurementUnit {
    fn from(x: RawMeasurementUnit) -> Self {
        MeasurementUnit { short: x.short, long: x.long }
    }
}

struct UnitSource {
    src: String,
    width: String,
    height: String,
}

impl From<RawUnitSource> for UnitSource {
    fn from(x: RawUnitSource) -> Self {
        UnitSource {
            src: x.src,
            width: x.width,
            height: x.height,
        }
    }
}

struct Image {
    source: String,
    alt: String,
    title: String,
    width: i32,
    height: i32,
    image_type: Option<String>,
    themes: Option<String>,
    color_invertable: bool,
    content_type: Option<String>,
}

impl From<RawImage> for Image {
    fn from(x: RawImage) -> Self {
        Image {
            source: x.src,
            alt: x.alt,
            title: x.title,
            width: x.width.get_val(),
            height: x.height.get_val(),
            image_type: x.r#type,
            themes: x.themes,
            color_invertable: x.colorinvertable.unwrap_or(false),
            content_type: x.contenttype,
        }
    }
}

struct Link {
    title: Option<String>,
    url: String,
    text: String,
}

impl From<RawLink> for Link {
    fn from(x: RawLink) -> Self {
        Link {
            title: x.title,
            url: x.url,
            text: x.text,
        }
    }
}

enum State {
    Single(SingleState),
    Multi(MultiState),
}

impl From<RawStateWrapper> for State {
    fn from(x: RawStateWrapper) -> Self {
        match x {
            RawStateWrapper::Single(y) => Self::Single(y.into()),
            RawStateWrapper::Multi(y) => Self::Multi(y.into()),
        }
    }
}

struct SingleState {
    name: String,
    input: String,
    stepbystep: bool,
}

impl From<RawState> for SingleState {
    fn from(x: RawState) -> Self {
        SingleState {
            name: x.name,
            input: x.input,
            stepbystep: x.stepbystep.unwrap_or(false),
        }
    }
}

struct MultiState {
    count: i32,
    value: String,
    delimiters: String,
    states: Vec<SingleState>,
}

impl From<RawMultiState> for MultiState {
    fn from(x: RawMultiState) -> Self {
        MultiState {
            count: x.count,
            value: x.value,
            delimiters: x.delimiters,
            states: x.states.into_iter().map(Into::into).collect(),
        }
    }
}

pub mod raw_response;

pub struct QueryResult {
    success: bool,

}

pub struct Pod {
    title: String,
    scanner: String,
    id: String,
    position: i32,
    error: bool,
    subpods: Vec<SubPod>,
    expressiontypes: Vec<ExpressionType>,
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
    infos: Vec<Info>
}

pub struct Definition {
    word: String, 
    description: String,
}

pub struct Info {
    units: Vec<Unit>,
    text: Option<String>,
    img: Option<Image>,
    links: Vec<Link>,
}

enum Unit {
    Time(TimeUnit),
    Source(UnitSource),
}
struct TimeUnit {
    short: String,
    long: String,
}

struct UnitSource {
    src: String,
    width: String,
    height: String,
}

struct Image {
    src: String,
    alt: String,
    title: String,
    width: i32,
    height: i32,
    image_type: Option<String>,
    themes: Option<String>,
    color_invertable: bool,
    content_type: Option<String>,
}

struct Link {
    title: Option<String>,
    url: String,
    text: String,
}

enum State {
    Single(SingleState),
    Multi(MultiState),
}

struct SingleState {
    name: String,
    input: String,
    stepbystep: bool,
}

struct MultiState {
    count: i32,
    value: String,
    delimiters: String,
    states: Vec<State>,
}

struct ExpressionType {
    name: String,
}
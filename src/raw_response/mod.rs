use serde::{Deserialize, Serialize};
use serde_json::Result;

mod pod;
use pod::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FullApiResponseError {
    code: String,
    msg: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ErrorWrapper {
    Ok(bool),
    Error(FullApiResponseError),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct QueryResult {
    success: bool,
    numpods: i32,
    datatypes: String,
    timedout: String,
    timedoutpods: String,
    timing: f32,
    parsetiming: f32,
    parsetimedout: bool,
    recalculate: String,
    id: String,
    parseidserver: Option<String>,
    host: String,
    server: String,
    related: String,
    version: String,
    inputstring: String,
    pods: Option<Vec<Pod>>,
    sources: Option<SourcesWrapper>,
    assumptions: Option<Vec<Assumption>>,
    error: ErrorWrapper,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assumption {
    r#type: String,
    word: Option<String>,
    desc: Option<String>,
    current: Option<String>,
    template: Option<String>,
    count: i32,
    values: AssumptionValuesWrapper,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum AssumptionValuesWrapper {
    Single(AssumptionValue),
    Multi(Vec<AssumptionValue>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssumptionValue {
    name: String,
    desc: String,
    valid: Option<BoolOrText>,
    input: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum BoolOrText {
    Bool(bool),
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum SourcesWrapper {
    Single(Source),
    Multi(Vec<Source>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    url: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RawApiResponse {
    queryresult: QueryResult,
}

#[test]
fn raw_api_response_deserialize() {
    let a: RawApiResponse = serde_json::from_str(
        r#"{
	"queryresult":{
		"success":false,
		"numpods":0,
		"datatypes":"",
		"timedout":"",
		"timedoutpods":"",
		"timing":1.9e-2,
		"parsetiming":0.0,
		"parsetimedout":false,
		"recalculate":"",
		"id":"",
		"parseidserver":"5",
		"host":"https:\/\/www6b3.wolframalpha.com",
		"server":"5",
		"related":"",
		"version":"2.6",
		"inputstring":"population france",
		"error":{
			"code":"1",
			"msg":"Invalid appid"
		}
	}
}"#,
    )
    .unwrap();

    let b: RawApiResponse = serde_json::from_str(
r#"{
	"queryresult":{
		"success":true,
		"error":false,
		"numpods":8,
		"datatypes":"City,Country",
		"timedout":"",
		"timedoutpods":"",
		"timing":2.507,
		"parsetiming":0.153,
		"parsetimedout":false,
		"recalculate":"",
		"id":"MSP8893186885ghge360i170000348ee64i84058ib8",
		"host":"https:\/\/www6b3.wolframalpha.com",
		"server":"13",
		"related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa8894186885ghge360i1700005c82257d61176ihh4222294402565530544",
		"version":"2.6",
		"inputstring":"population france",
		"pods":[
			{
				"title":"Input interpretation",
				"scanner":"Identity",
				"id":"Input",
				"position":100,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP8895186885ghge360i17000017e20d850ch73bd1?MSPStoreType=image\/gif&s=13",
							"alt":"France | population",
							"title":"France | population",
							"width":146,
							"height":23,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"France | population"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				}
			},
			{
				"title":"Result",
				"scanner":"Data",
				"id":"Result",
				"position":200,
				"error":false,
				"numsubpods":1,
				"primary":true,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"CountryData"
						},
						"datasources":{
							"datasource":"UNData"
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP8896186885ghge360i1700001c1b70ff10793708?MSPStoreType=image\/gif&s=13",
							"alt":"65.3 million people (world rank: 22nd) (2020 estimate)",
							"title":"65.3 million people (world rank: 22nd) (2020 estimate)",
							"width":332,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"65.3 million people (world rank: 22nd) (2020 estimate)"
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			},
			{
				"title":"Recent population history",
				"scanner":"Data",
				"id":"RecentHistory:Population:CountryData",
				"position":300,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"CountryData"
						},
						"datasources":{
							"datasource":"UNData"
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP8897186885ghge360i1700003406hg4f8f5hi9fi?MSPStoreType=image\/gif&s=13",
							"alt":"Recent population history",
							"title":"",
							"width":383,
							"height":160,
							"type":"TimeSeriesPlot_1",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":""
					}
				],
				"expressiontypes":{
					"name":"Default"
				},
				"states":[
					{
						"name":"Show projections",
						"input":"RecentHistory:Population:CountryData__Show projections"
					},
					{
						"name":"Log scale",
						"input":"RecentHistory:Population:CountryData__Log scale"
					}
				]
			},
			{
				"title":"Long-term population history",
				"scanner":"Data",
				"id":"LongTermHistory:Population:CountryData",
				"position":400,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"CountryData"
						},
						"datasources":{
							"datasource":"UNData"
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP8898186885ghge360i1700004c839g212ab5891h?MSPStoreType=image\/gif&s=13",
							"alt":"Long-term population history",
							"title":"",
							"width":546,
							"height":230,
							"type":"TimeSeriesPlot_1",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":""
					}
				],
				"expressiontypes":{
					"name":"Default"
				},
				"states":[
					{
						"name":"Show projections",
						"input":"LongTermHistory:Population:CountryData__Show projections"
					},
					{
						"name":"Log scale",
						"input":"LongTermHistory:Population:CountryData__Log scale"
					}
				]
			},
			{
				"title":"Demographics",
				"scanner":"Data",
				"id":"DemographicProperties:CountryData",
				"position":500,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"CountryData"
						},
						"datasources":{
							"datasource":[
								"UNData",
								"CIAFactbook"
							]
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP8899186885ghge360i17000010b0ii2hda0h4eb9?MSPStoreType=image\/gif&s=13",
							"alt":"population | 65.3 million people (world rank: 22nd) (2020 estimate)\npopulation density | 119 people\/km^2 (people per square kilometer) (world rank: 100th) (2019 estimate)\npopulation growth | 0.14 %\/yr (world rank: 198th) (2019 estimate)\nlife expectancy | 82.4 years (world rank: 20th) (2021 estimate)\nmedian age | 41.2 years (world rank: 26th) (2015 estimate)",
							"title":"population | 65.3 million people (world rank: 22nd) (2020 estimate)\npopulation density | 119 people\/km^2 (people per square kilometer) (world rank: 100th) (2019 estimate)\npopulation growth | 0.14 %\/yr (world rank: 198th) (2019 estimate)\nlife expectancy | 82.4 years (world rank: 20th) (2021 estimate)\nmedian age | 41.2 years (world rank: 26th) (2015 estimate)",
							"width":546,
							"height":187,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"population | 65.3 million people (world rank: 22nd) (2020 estimate)\npopulation density | 119 people\/km^2 (people per square kilometer) (world rank: 100th) (2019 estimate)\npopulation growth | 0.14 %\/yr (world rank: 198th) (2019 estimate)\nlife expectancy | 82.4 years (world rank: 20th) (2021 estimate)\nmedian age | 41.2 years (world rank: 26th) (2015 estimate)"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				},
				"states":[
					{
						"name":"Show rates",
						"input":"DemographicProperties:CountryData__Show rates"
					},
					{
						"name":"Show distribution",
						"input":"DemographicProperties:CountryData__Show distribution"
					},
					{
						"name":"Show non-metric",
						"input":"DemographicProperties:CountryData__Show non-metric"
					}
				],
				"infos":{
					"units":[
						{
							"short":"%\/yr",
							"long":"percent per year"
						},
						{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP8900186885ghge360i17000063feh8ed72e9aee5?MSPStoreType=image\/gif&s=13",
							"width":"164",
							"height":"27"
						}
					]
				}
			},
			{
				"title":"Age distribution",
				"scanner":"Data",
				"id":"AgeDistributionPyramidGraphic:AgeDistributionData",
				"position":600,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP8901186885ghge360i170000246c1658be21de2d?MSPStoreType=image\/gif&s=13",
							"alt":"Age distribution",
							"title":"",
							"width":546,
							"height":467,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":""
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			},
			{
				"title":"Largest cities",
				"scanner":"Data",
				"id":"LargestCities:CountryData",
				"position":700,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":[
								"CityData",
								"CountryData"
							]
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP8902186885ghge360i1700001hgi73egh8050dg1?MSPStoreType=image\/gif&s=13",
							"alt":"city | population\nParis, Ile-de-France | 2.206 million people\nMarseille, Provence-Alpes-Cote-d'Azur | 855393 people\nLyon, Auvergne-Rhône-Alpes | 506615 people\nToulouse, Occitanie | 471941 people\nNice, Provence-Alpes-Cote-d'Azur | 342522 people\n(2004, 2007, 2013, 2014, 2015, and 2017 estimates)",
							"title":"city | population\nParis, Ile-de-France | 2.206 million people\nMarseille, Provence-Alpes-Cote-d'Azur | 855393 people\nLyon, Auvergne-Rhône-Alpes | 506615 people\nToulouse, Occitanie | 471941 people\nNice, Provence-Alpes-Cote-d'Azur | 342522 people\n(2004, 2007, 2013, 2014, 2015, and 2017 estimates)",
							"width":429,
							"height":207,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"city | population\nParis, Ile-de-France | 2.206 million people\nMarseille, Provence-Alpes-Cote-d'Azur | 855393 people\nLyon, Auvergne-Rhône-Alpes | 506615 people\nToulouse, Occitanie | 471941 people\nNice, Provence-Alpes-Cote-d'Azur | 342522 people\n(2004, 2007, 2013, 2014, 2015, and 2017 estimates)"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				},
				"states":[
					{
						"name":"More",
						"input":"LargestCities:CountryData__More"
					}
				]
			},
			{
				"title":"Comparisons",
				"scanner":"Unit",
				"id":"ComparisonAsPersonCount",
				"position":800,
				"error":false,
				"numsubpods":2,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP8903186885ghge360i1700003cg4ag84g3d7d322?MSPStoreType=image\/gif&s=13",
							"alt":" ≈ 0.62 × current population of Philippines ( 104.9 million people )",
							"title":" ≈ 0.62 × current population of Philippines ( 104.9 million people )",
							"width":390,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":" ≈ 0.62 × current population of Philippines ( 104.9 million people )"
					},
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP8904186885ghge360i1700004c7e0g0dfeb9fedf?MSPStoreType=image\/gif&s=13",
							"alt":" ≈ 0.79 × current population of Germany ( 82.11 million people )",
							"title":" ≈ 0.79 × current population of Germany ( 82.11 million people )",
							"width":379,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":" ≈ 0.79 × current population of Germany ( 82.11 million people )"
					}
				],
				"expressiontypes":[
					{
						"name":"Default"
					},
					{
						"name":"Default"
					}
				]
			}
		],
		"sources":[
			{
				"url":"https:\/\/www6b3.wolframalpha.com\/sources\/CityDataSourceInformationNotes.html",
				"text":"City data"
			},
			{
				"url":"https:\/\/www6b3.wolframalpha.com\/sources\/CountryDataSourceInformationNotes.html",
				"text":"Country data"
			}
		]
	}
}"#
    ).unwrap();

    let c: RawApiResponse = serde_json::from_str(
r#"{
	"queryresult":{
		"success":true,
		"error":false,
		"numpods":7,
		"datatypes":"PlanetaryMoon",
		"timedout":"",
		"timedoutpods":"",
		"timing":1.5030000000000001,
		"parsetiming":0.23800000000000002,
		"parsetimedout":false,
		"recalculate":"",
		"id":"MSP58431cg68e4id1204i8g000036i54b9i35ecd9b5",
		"host":"https:\/\/www6b3.wolframalpha.com",
		"server":"19",
		"related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa58441cg68e4id1204i8g000043266h35di322bdf7179915770051379692",
		"version":"2.6",
		"inputstring":"distance to moon",
		"pods":[
			{
				"title":"Input interpretation",
				"scanner":"Identity",
				"id":"Input",
				"position":100,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP58451cg68e4id1204i8g00000d6h70f103f38cda?MSPStoreType=image\/gif&s=19",
							"alt":"Moon | distance from Earth",
							"title":"Moon | distance from Earth",
							"width":199,
							"height":23,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"Moon | distance from Earth"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				}
			},
			{
				"title":"Current result",
				"scanner":"Data",
				"id":"Result",
				"position":200,
				"error":false,
				"numsubpods":1,
				"primary":true,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"PlanetaryMoonData"
						},
						"datasources":{
							"datasource":[
								"AllensAstrophysicalQuantities",
								"AstronomicalAlgorithms",
								"PlanetaryTheoriesInRectangularAndSphericalVariables",
								"SolarSystemDynamics"
							]
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP58461cg68e4id1204i8g000046c2h0ia4006i7c4?MSPStoreType=image\/gif&s=19",
							"alt":"398956 km (kilometers)",
							"title":"398956 km (kilometers)",
							"width":158,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"398956 km (kilometers)"
					}
				],
				"expressiontypes":{
					"name":"Default"
				},
				"states":[
					{
						"name":"Show non-metric",
						"input":"Result__Show non-metric"
					}
				]
			},
			{
				"title":"History",
				"scanner":"Data",
				"id":"DistanceHistory:PlanetaryMoonData",
				"position":300,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"PlanetaryMoonData"
						},
						"datasources":{
							"datasource":[
								"AllensAstrophysicalQuantities",
								"AstronomicalAlgorithms",
								"PlanetaryTheoriesInRectangularAndSphericalVariables",
								"SolarSystemDynamics"
							]
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP58471cg68e4id1204i8g00004b1a641g4i57cd79?MSPStoreType=image\/gif&s=19",
							"alt":"History",
							"title":"",
							"width":546,
							"height":201,
							"type":"TimeSeriesPlot_1",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":""
					}
				],
				"expressiontypes":{
					"name":"Default"
				},
				"states":[
					{
						"name":"Use Imperial",
						"input":"DistanceHistory:PlanetaryMoonData__Use Imperial"
					},
					{
						"count":5,
						"value":"±3 months",
						"delimiters":"",
						"states":[
							{
								"name":"±3 months",
								"input":"DistanceHistory:PlanetaryMoonData__±3 months"
							},
							{
								"name":"±6 months",
								"input":"DistanceHistory:PlanetaryMoonData__±6 months"
							},
							{
								"name":"±1 year",
								"input":"DistanceHistory:PlanetaryMoonData__±1 year"
							},
							{
								"name":"±5 years",
								"input":"DistanceHistory:PlanetaryMoonData__±5 years"
							},
							{
								"name":"±10 years",
								"input":"DistanceHistory:PlanetaryMoonData__±10 years"
							}
						]
					}
				]
			},
			{
				"title":"Unit conversions",
				"scanner":"Unit",
				"id":"UnitConversion",
				"position":400,
				"error":false,
				"numsubpods":2,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP58481cg68e4id1204i8g000060dc86ab0fgeb0e5?MSPStoreType=image\/gif&s=19",
							"alt":"247899 miles",
							"title":"247899 miles",
							"width":89,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"247899 miles"
					},
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP58491cg68e4id1204i8g00001agd4bcd1cg6i52e?MSPStoreType=image\/gif&s=19",
							"alt":"3.99×10^8 meters",
							"title":"3.99×10^8 meters",
							"width":114,
							"height":21,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"3.99×10^8 meters"
					}
				],
				"expressiontypes":[
					{
						"name":"Default"
					},
					{
						"name":"Default"
					}
				]
			},
			{
				"title":"Comparison as distance",
				"scanner":"Unit",
				"id":"ComparisonAsDistance",
				"position":500,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP58501cg68e4id1204i8g00003afe1c6fe8ef6572?MSPStoreType=image\/gif&s=19",
							"alt":" ≈ mean Moon-Earth distance ( 3.85×10^8 m )",
							"title":" ≈ mean Moon-Earth distance ( 3.85×10^8 m )",
							"width":276,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":" ≈ mean Moon-Earth distance ( 3.85×10^8 m )"
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			},
			{
				"title":"Corresponding quantities",
				"scanner":"Unit",
				"id":"CorrespondingQuantity",
				"position":600,
				"error":false,
				"numsubpods":2,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP58511cg68e4id1204i8g000038c9h4a7gb34f6i1?MSPStoreType=image\/gif&s=19",
							"alt":"Light travel time t in vacuum from t = x\/c:\n | 1.3 seconds",
							"title":"Light travel time t in vacuum from t = x\/c:\n | 1.3 seconds",
							"width":283,
							"height":43,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"Light travel time t in vacuum from t = x\/c:\n | 1.3 seconds"
					},
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP58521cg68e4id1204i8g00004e83g6bc613f80ec?MSPStoreType=image\/gif&s=19",
							"alt":"Light travel time t in an optical fiber t = 1.48x\/c:\n | 2 seconds",
							"title":"Light travel time t in an optical fiber t = 1.48x\/c:\n | 2 seconds",
							"width":321,
							"height":43,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"Light travel time t in an optical fiber t = 1.48x\/c:\n | 2 seconds"
					}
				],
				"expressiontypes":[
					{
						"name":"Default"
					},
					{
						"name":"Default"
					}
				]
			},
			{
				"title":"Orbital properties",
				"scanner":"Data",
				"id":"BasicMoonOrbitalProperties:PlanetaryMoonData",
				"position":700,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"PlanetaryMoonData"
						},
						"datasources":{
							"datasource":[
								"AllensAstrophysicalQuantities",
								"NationalSpaceScienceDataCenter",
								"SolarSystemDynamics",
								"AstronomicalAlgorithms",
								"PlanetaryTheoriesInRectangularAndSphericalVariables"
							]
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP58531cg68e4id1204i8g000022hca109dc72146h?MSPStoreType=image\/gif&s=19",
							"alt":"current distance from Earth | 398956 km\n62.55 a\naverage distance from Earth | 385000 km\n60.36 a\norbital period | 27.322 days",
							"title":"current distance from Earth | 398956 km\n62.55 a\naverage distance from Earth | 385000 km\n60.36 a\norbital period | 27.322 days",
							"width":312,
							"height":145,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"current distance from Earth | 398956 km\n62.55 a\naverage distance from Earth | 385000 km\n60.36 a\norbital period | 27.322 days"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				},
				"states":[
					{
						"name":"Show non-metric",
						"input":"BasicMoonOrbitalProperties:PlanetaryMoonData__Show non-metric"
					},
					{
						"name":"More",
						"input":"BasicMoonOrbitalProperties:PlanetaryMoonData__More"
					},
					{
						"name":"Show history",
						"input":"BasicMoonOrbitalProperties:PlanetaryMoonData__Show history"
					}
				],
				"infos":{
					"units":[
						[
							{
								"short":"a",
								"long":"Earth equatorial radii"
							},
							{
								"short":"km",
								"long":"kilometers"
							}
						],
						{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP58541cg68e4id1204i8g00005cada8iaeaha82ei?MSPStoreType=image\/gif&s=19",
							"width":"181",
							"height":"49"
						}
					]
				}
			}
		],
		"sources":{
			"url":"https:\/\/www6b3.wolframalpha.com\/sources\/PlanetaryMoonDataSourceInformationNotes.html",
			"text":"Planetary moon data"
		}
	}
}"#
    ).unwrap();

    let d: RawApiResponse = serde_json::from_str(
r#"{
	"queryresult":{
		"success":true,
		"error":false,
		"numpods":4,
		"datatypes":"Formula",
		"timedout":"",
		"timedoutpods":"",
		"timing":0.615,
		"parsetiming":0.19,
		"parsetimedout":false,
		"recalculate":"",
		"id":"MSP5548147721i6ag3bd3a2000056929ch5db673g50",
		"host":"https:\/\/www6b3.wolframalpha.com",
		"server":"16",
		"related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa5549147721i6ag3bd3a2000029cgdff9185ebgi28350669649384844499",
		"version":"2.6",
		"inputstring":"self inductance of coaxial cable",
		"pods":[
			{
				"title":"Input interpretation",
				"scanner":"Formula",
				"id":"Input",
				"position":100,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP5550147721i6ag3bd3a200005b0eg078572bh438?MSPStoreType=image\/gif&s=16",
							"alt":"self-inductance of a single-core cable",
							"title":"self-inductance of a single-core cable",
							"width":260,
							"height":23,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"self-inductance of a single-core cable"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				}
			},
			{
				"title":"Equation",
				"scanner":"Formula",
				"id":"Equation",
				"position":200,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP5551147721i6ag3bd3a200005fi6e6326hg436i5?MSPStoreType=image\/gif&s=16",
							"alt":"L = (μ_0 l log(R_1\/R_2))\/(2 π) | \nL | self-inductance\nR_2 | conductor radius\nR_1 | sheath radius\nl | length\nμ_0 | magnetic constant (≈ 1.256637062×10^-6 H\/m)\n(assuming high frequency currents)",
							"title":"L = (μ_0 l log(R_1\/R_2))\/(2 π) | \nL | self-inductance\nR_2 | conductor radius\nR_1 | sheath radius\nl | length\nμ_0 | magnetic constant (≈ 1.256637062×10^-6 H\/m)\n(assuming high frequency currents)",
							"width":371,
							"height":244,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"L = (μ_0 l log(R_1\/R_2))\/(2 π) | \nL | self-inductance\nR_2 | conductor radius\nR_1 | sheath radius\nl | length\nμ_0 | magnetic constant (≈ 1.256637062×10^-6 H\/m)\n(assuming high frequency currents)"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				},
				"infos":{
					"text":"log(x) is the natural logarithm",
					"img":{
						"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP5552147721i6ag3bd3a200005cg96ih070aa299d?MSPStoreType=image\/gif&s=16",
						"alt":"log(x) is the natural logarithm",
						"title":"log(x) is the natural logarithm",
						"width":"198",
						"height":"19"
					},
					"links":[
						{
							"url":"http:\/\/reference.wolfram.com\/language\/ref\/Log.html",
							"text":"Documentation",
							"title":"Mathematica"
						},
						{
							"url":"http:\/\/functions.wolfram.com\/ElementaryFunctions\/Log",
							"text":"Properties",
							"title":"Wolfram Functions Site"
						},
						{
							"url":"http:\/\/mathworld.wolfram.com\/NaturalLogarithm.html",
							"text":"Definition",
							"title":"MathWorld"
						}
					]
				}
			},
			{
				"title":"Input values",
				"scanner":"Formula",
				"id":"InputValue",
				"position":300,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP5553147721i6ag3bd3a200000eif8g2ibac6cce2?MSPStoreType=image\/gif&s=16",
							"alt":"conductor radius | 1 cm (centimeter)\nsheath radius | 2.5 cm (centimeters)\nlength | 5 meters",
							"title":"conductor radius | 1 cm (centimeter)\nsheath radius | 2.5 cm (centimeters)\nlength | 5 meters",
							"width":295,
							"height":103,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"conductor radius | 1 cm (centimeter)\nsheath radius | 2.5 cm (centimeters)\nlength | 5 meters"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				}
			},
			{
				"title":"Result",
				"scanner":"Formula",
				"id":"Result",
				"position":400,
				"error":false,
				"numsubpods":1,
				"primary":true,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP5554147721i6ag3bd3a200002fg44da85518chg8?MSPStoreType=image\/gif&s=16",
							"alt":"self-inductance | 916.3 nH (nanohenries)\n= 0.9163 μH (microhenries)\n= 9.163×10^-7 H (henries)",
							"title":"self-inductance | 916.3 nH (nanohenries)\n= 0.9163 μH (microhenries)\n= 9.163×10^-7 H (henries)",
							"width":327,
							"height":83,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"self-inductance | 916.3 nH (nanohenries)\n= 0.9163 μH (microhenries)\n= 9.163×10^-7 H (henries)"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				},
				"states":[
					{
						"name":"Step-by-step solution",
						"input":"Result__Step-by-step solution",
						"stepbystep":true
					}
				]
			}
		],
		"assumptions":[
			{
				"type":"FormulaVariable",
				"desc":"conductor radius",
				"current":"1",
				"count":1,
				"values":{
					"name":"CoaxialCableSelfInductance.R2",
					"desc":"1 cm",
					"valid":true,
					"input":"*F.CoaxialCableSelfInductance.R2-_1+cm"
				}
			},
			{
				"type":"FormulaVariable",
				"desc":"sheath radius",
				"current":"1",
				"count":1,
				"values":{
					"name":"CoaxialCableSelfInductance.R1",
					"desc":"2.5 cm",
					"valid":true,
					"input":"*F.CoaxialCableSelfInductance.R1-_2.5+cm"
				}
			},
			{
				"type":"FormulaVariable",
				"desc":"length",
				"current":"1",
				"count":1,
				"values":{
					"name":"CoaxialCableSelfInductance.l",
					"desc":"5 m",
					"valid":true,
					"input":"*F.CoaxialCableSelfInductance.l-_5+m"
				}
			}
		]
	}
}"#
    ).unwrap();

    let e: RawApiResponse = serde_json::from_str(
r#"{
	"queryresult":{
		"success":true,
		"error":false,
		"numpods":9,
		"datatypes":"MatrixExp",
		"timedout":"",
		"timedoutpods":"",
		"timing":1.801,
		"parsetiming":0.111,
		"parsetimedout":false,
		"recalculate":"",
		"id":"MSP60216cbcff594hh2cbb000064f4bc30idcach8d",
		"host":"https:\/\/www6b3.wolframalpha.com",
		"server":"20",
		"related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa60316cbcff594hh2cbb000019h7i6cf2i11i1488458888475783217078",
		"version":"2.6",
		"inputstring":"matrix exponential",
		"pods":[
			{
				"title":"Input interpretation",
				"scanner":"Identity",
				"id":"Input",
				"position":100,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP60416cbcff594hh2cbb00003i53048i20be84i2?MSPStoreType=image\/gif&s=20",
							"alt":"Matrix exponential (1.2 | 5.6\n3 | 4)",
							"title":"Matrix exponential (1.2 | 5.6\n3 | 4)",
							"width":251,
							"height":54,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"Matrix exponential (1.2 | 5.6\n3 | 4)"
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			},
			{
				"title":"Result",
				"scanner":"FormulaMatrix",
				"id":"Result",
				"position":200,
				"error":false,
				"numsubpods":1,
				"primary":true,
				"subpods":[
					{
						"title":"",
						"primary":true,
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP60516cbcff594hh2cbb00005g26001257gef21b?MSPStoreType=image\/gif&s=20",
							"alt":"(346.557 | 661.735\n354.501 | 677.425)",
							"title":"(346.557 | 661.735\n354.501 | 677.425)",
							"width":132,
							"height":38,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"(346.557 | 661.735\n354.501 | 677.425)"
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			},
			{
				"title":"Dimensions",
				"scanner":"FormulaMatrix",
				"id":"Dimensions",
				"position":300,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP60616cbcff594hh2cbb00002i56a88ca1g1abb0?MSPStoreType=image\/gif&s=20",
							"alt":"2 (rows) × 2 (columns)",
							"title":"2 (rows) × 2 (columns)",
							"width":150,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"2 (rows) × 2 (columns)"
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			},
			{
				"title":"Matrix plot",
				"scanner":"FormulaMatrix",
				"id":"MatrixPlot",
				"position":400,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP60716cbcff594hh2cbb000018f9051b0aa055a3?MSPStoreType=image\/gif&s=20",
							"alt":"Matrix plot",
							"title":"",
							"width":75,
							"height":89,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":false,
							"contenttype":"image\/gif"
						},
						"plaintext":""
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			},
			{
				"title":"Trace",
				"scanner":"FormulaMatrix",
				"id":"Trace",
				"position":500,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP60816cbcff594hh2cbb000028841551023197i1?MSPStoreType=image\/gif&s=20",
							"alt":"1023.98",
							"title":"1023.98",
							"width":52,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"1023.98"
					}
				],
				"expressiontypes":{
					"name":"Default"
				},
				"states":[
					{
						"name":"Step-by-step solution",
						"input":"Trace__Step-by-step solution",
						"stepbystep":true
					}
				]
			},
			{
				"title":"Determinant",
				"scanner":"FormulaMatrix",
				"id":"Determinant",
				"position":600,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP60916cbcff594hh2cbb0000240bg1igg70a0hid?MSPStoreType=image\/gif&s=20",
							"alt":"181.272",
							"title":"181.272",
							"width":52,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"181.272"
					}
				],
				"expressiontypes":{
					"name":"Default"
				},
				"states":[
					{
						"name":"Step-by-step solution",
						"input":"Determinant__Step-by-step solution",
						"stepbystep":true
					}
				]
			},
			{
				"title":"Inverse",
				"scanner":"FormulaMatrix",
				"id":"Inverse",
				"position":700,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP61016cbcff594hh2cbb00003gc3hc054i8bd218?MSPStoreType=image\/gif&s=20",
							"alt":"(3.73706 | -3.6505\n-1.95563 | 1.91181)",
							"title":"(3.73706 | -3.6505\n-1.95563 | 1.91181)",
							"width":146,
							"height":38,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"(3.73706 | -3.6505\n-1.95563 | 1.91181)"
					}
				],
				"expressiontypes":{
					"name":"Default"
				},
				"states":[
					{
						"name":"Step-by-step solution",
						"input":"Inverse__Step-by-step solution",
						"stepbystep":true
					}
				]
			},
			{
				"title":"Diagonalization",
				"scanner":"FormulaMatrix",
				"id":"Diagonalization",
				"position":800,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP61116cbcff594hh2cbb00000fhbd732fh4354ed?MSPStoreType=image\/gif&s=20",
							"alt":"M = S.J.S^(-1)\nwhere\nM = (346.557 | 661.735\n354.501 | 677.425)\nS = (-1.91043 | 0.977094\n1 | 1)\nJ = (0.177057 | 0\n0 | 1023.81)\nS^(-1) = (-0.346318 | 0.338385\n0.346318 | 0.661615)",
							"title":"M = S.J.S^(-1)\nwhere\nM = (346.557 | 661.735\n354.501 | 677.425)\nS = (-1.91043 | 0.977094\n1 | 1)\nJ = (0.177057 | 0\n0 | 1023.81)\nS^(-1) = (-0.346318 | 0.338385\n0.346318 | 0.661615)",
							"width":201,
							"height":251,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"M = S.J.S^(-1)\nwhere\nM = (346.557 | 661.735\n354.501 | 677.425)\nS = (-1.91043 | 0.977094\n1 | 1)\nJ = (0.177057 | 0\n0 | 1023.81)\nS^(-1) = (-0.346318 | 0.338385\n0.346318 | 0.661615)"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				}
			},
			{
				"title":"Condition number",
				"scanner":"FormulaMatrix",
				"id":"ConditionNumber",
				"position":900,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP61216cbcff594hh2cbb00004f593bh1gb43ch22?MSPStoreType=image\/gif&s=20",
							"alt":"7623.41",
							"title":"7623.41",
							"width":52,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"7623.41"
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			}
		],
		"assumptions":[
			{
				"type":"Clash",
				"word":"matrix exponential",
				"template":"Assuming \"${word}\" is ${desc1}. Use as ${desc2} instead",
				"count":3,
				"values":[
					{
						"name":"Calculator",
						"desc":"a computation",
						"input":"*C.matrix+exponential-_*Calculator-"
					},
					{
						"name":"MathWorld",
						"desc":" referring to a mathematical definition",
						"input":"*C.matrix+exponential-_*MathWorld-"
					},
					{
						"name":"SloppyFunction",
						"desc":"a function",
						"input":"*C.matrix+exponential-_*SloppyFunction-"
					}
				]
			},
			{
				"type":"FormulaVariable",
				"desc":"matrix",
				"current":"1",
				"count":1,
				"values":{
					"name":"MatrixExponentialCalculator.matrixexp",
					"desc":"{{1.2, 5.6}, {3, 4}}",
					"valid":true,
					"input":"*F.MatrixExponentialCalculator.matrixexp-_%7B%7B1.2%2C+5.6%7D%2C+%7B3%2C+4%7D%7D"
				}
			}
		]
	}
}"#
    ).unwrap();

    let f: RawApiResponse = serde_json::from_str(
r#"{
	"queryresult":{
		"success":true,
		"error":false,
		"numpods":4,
		"datatypes":"",
		"timedout":"",
		"timedoutpods":"",
		"timing":0.711,
		"parsetiming":0.183,
		"parsetimedout":false,
		"recalculate":"",
		"id":"MSP125614772c5c2ed8666700001ebc2d5919efg7g7",
		"host":"https:\/\/www6b3.wolframalpha.com",
		"server":"16",
		"related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa125714772c5c2ed8666700002612bc9d6c22df438350669649384844499",
		"version":"2.6",
		"inputstring":"oort cloud radius",
		"pods":[
			{
				"title":"Input interpretation",
				"scanner":"Identity",
				"id":"Input",
				"position":100,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP125814772c5c2ed86667000030b810ceii6ib4be?MSPStoreType=image\/gif&s=16",
							"alt":"Oort cloud radius",
							"title":"Oort cloud radius",
							"width":115,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"Oort cloud radius"
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			},
			{
				"title":"Result",
				"scanner":"Data",
				"id":"Result",
				"position":200,
				"error":false,
				"numsubpods":1,
				"primary":true,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP125914772c5c2ed866670000456619dh9h66i327?MSPStoreType=image\/gif&s=16",
							"alt":"(1900 to 210000) au (astronomical units)",
							"title":"(1900 to 210000) au (astronomical units)",
							"width":271,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"(1900 to 210000) au (astronomical units)"
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			},
			{
				"title":"Unit conversions",
				"scanner":"Unit",
				"id":"UnitConversion",
				"position":300,
				"error":false,
				"numsubpods":5,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP126014772c5c2ed86667000061h5694ei46ebgi3?MSPStoreType=image\/gif&s=16",
							"alt":"≈ (2.8×10^11 to 3.1×10^13) km (kilometers)",
							"title":"≈ (2.8×10^11 to 3.1×10^13) km (kilometers)",
							"width":283,
							"height":24,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"≈ (2.8×10^11 to 3.1×10^13) km (kilometers)"
					},
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP126114772c5c2ed8666700003gecf39bhdi40bh8?MSPStoreType=image\/gif&s=16",
							"alt":"≈ (2.8×10^14 to 3.1×10^16) meters",
							"title":"≈ (2.8×10^14 to 3.1×10^16) meters",
							"width":224,
							"height":24,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"≈ (2.8×10^14 to 3.1×10^16) meters"
					},
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP126214772c5c2ed86667000047i47ih5e95ach8h?MSPStoreType=image\/gif&s=16",
							"alt":"≈ (260 to 29000) light hours",
							"title":"≈ (260 to 29000) light hours",
							"width":187,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"≈ (260 to 29000) light hours"
					},
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP126314772c5c2ed8666700002399dci2fb4g4g9i?MSPStoreType=image\/gif&s=16",
							"alt":"≈ (170 billion to 19 trillion) miles",
							"title":"≈ (170 billion to 19 trillion) miles",
							"width":216,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"≈ (170 billion to 19 trillion) miles"
					},
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP126414772c5c2ed86667000039687415dg357629?MSPStoreType=image\/gif&s=16",
							"alt":"≈ (0.0091 to 1) pc (parsecs)",
							"title":"≈ (0.0091 to 1) pc (parsecs)",
							"width":183,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"≈ (0.0091 to 1) pc (parsecs)"
					}
				],
				"expressiontypes":[
					{
						"name":"Default"
					},
					{
						"name":"Default"
					},
					{
						"name":"Default"
					},
					{
						"name":"Default"
					},
					{
						"name":"Default"
					}
				]
			},
			{
				"title":"Corresponding quantities",
				"scanner":"Unit",
				"id":"CorrespondingQuantity",
				"position":400,
				"error":false,
				"numsubpods":2,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP126514772c5c2ed8666700004hc61bed8e2bg3e2?MSPStoreType=image\/gif&s=16",
							"alt":"Light travel time t in vacuum from t = x\/c:\n | (0.03 to 3.3) years",
							"title":"Light travel time t in vacuum from t = x\/c:\n | (0.03 to 3.3) years",
							"width":283,
							"height":43,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"Light travel time t in vacuum from t = x\/c:\n | (0.03 to 3.3) years"
					},
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP126614772c5c2ed86667000058ia6f57he4ai8da?MSPStoreType=image\/gif&s=16",
							"alt":"Light travel time t in an optical fiber t = 1.48x\/c:\n | (0.044 to 4.9) years",
							"title":"Light travel time t in an optical fiber t = 1.48x\/c:\n | (0.044 to 4.9) years",
							"width":321,
							"height":43,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"Light travel time t in an optical fiber t = 1.48x\/c:\n | (0.044 to 4.9) years"
					}
				],
				"expressiontypes":[
					{
						"name":"Default"
					},
					{
						"name":"Default"
					}
				]
			}
		]
	}
}"#
    ).unwrap();

    let g: RawApiResponse = serde_json::from_str(
r#"{
	"queryresult":{
		"success":true,
		"error":false,
		"numpods":5,
		"datatypes":"Formula",
		"timedout":"",
		"timedoutpods":"",
		"timing":1.7890000000000001,
		"parsetiming":0.612,
		"parsetimedout":false,
		"recalculate":"",
		"id":"MSP7581cg6975gg65bi9890000388hf97625egcf40",
		"host":"https:\/\/www6b3.wolframalpha.com",
		"server":"19",
		"related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa7591cg6975gg65bi98900001b46ged0e5b3ecie7179915770051379692",
		"version":"2.6",
		"inputstring":"mortgage $150,000, 6.5%, 30 years",
		"pods":[
			{
				"title":"Input information",
				"scanner":"Formula",
				"id":"Input",
				"position":100,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP7601cg6975gg65bi98900004e6a955b72efii85?MSPStoreType=image\/gif&s=19",
							"alt":"fixed rate mortgage | \nloan amount | $150000.00 (US dollars)\nannual percentage rate | 6.5%\nloan period | 30 years",
							"title":"fixed rate mortgage | \nloan amount | $150000.00 (US dollars)\nannual percentage rate | 6.5%\nloan period | 30 years",
							"width":360,
							"height":136,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"fixed rate mortgage | \nloan amount | $150000.00 (US dollars)\nannual percentage rate | 6.5%\nloan period | 30 years"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				}
			},
			{
				"title":"Monthly payments",
				"scanner":"Formula",
				"id":"MonthlyPayments",
				"position":200,
				"error":false,
				"numsubpods":1,
				"primary":true,
				"subpods":[
					{
						"title":"",
						"primary":true,
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP7611cg6975gg65bi98900005adif2eebe24110e?MSPStoreType=image\/gif&s=19",
							"alt":"monthly payment | $948\neffective interest rate | 6.697%",
							"title":"monthly payment | $948\neffective interest rate | 6.697%",
							"width":233,
							"height":70,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"monthly payment | $948\neffective interest rate | 6.697%"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				}
			},
			{
				"title":"Mortgage totals",
				"scanner":"Formula",
				"id":"MortgageTotals",
				"position":300,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP7621cg6975gg65bi98900005652294a3h899988?MSPStoreType=image\/gif&s=19",
							"alt":"principal paid | $150000\ntotal interest paid | $191317\ntotal payments | $341317 | ",
							"title":"principal paid | $150000\ntotal interest paid | $191317\ntotal payments | $341317 | ",
							"width":482,
							"height":104,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"principal paid | $150000\ntotal interest paid | $191317\ntotal payments | $341317 | "
					}
				],
				"expressiontypes":{
					"name":"Grid"
				}
			},
			{
				"title":"Payments and balances",
				"scanner":"Formula",
				"id":"PaymentsAndBalances",
				"position":400,
				"error":false,
				"numsubpods":2,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP7631cg6975gg65bi98900004e489bgb1e8202i7?MSPStoreType=image\/gif&s=19",
							"alt":"Payments and balances",
							"title":"",
							"width":385,
							"height":142,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":""
					},
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP7641cg6975gg65bi98900002c92b6519fae1094?MSPStoreType=image\/gif&s=19",
							"alt":"Payments and balances",
							"title":"",
							"width":323,
							"height":139,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":""
					}
				],
				"expressiontypes":[
					{
						"name":"Default"
					},
					{
						"name":"Default"
					}
				]
			},
			{
				"title":"Payments table",
				"scanner":"Formula",
				"id":"PaymentsTable",
				"position":500,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP7651cg6975gg65bi989000015ib0dedh7hf09f9?MSPStoreType=image\/gif&s=19",
							"alt":"year | monthly payment | ending balance | yearly principal paid | yearly interest paid\n1 | $948 | $148323 | $1677 | $9701\n2 | $948 | $146535 | $1789 | $9588\n3 | $948 | $144626 | $1909 | $9469\n4 | $948 | $142589 | $2037 | $9341\n5 | $948 | $140416 | $2173 | $9204\n6 | $948 | $138098 | $2318 | $9059\n7 | $948 | $135624 | $2474 | $8904\n8 | $948 | $132985 | $2639 | $8738\n9 | $948 | $130169 | $2816 | $8561\n10 | $948 | $127164 | $3005 | $8373\n11 | $948 | $123958 | $3206 | $8171\n12 | $948 | $120538 | $3421 | $7957\n13 | $948 | $116888 | $3650 | $7727\n14 | $948 | $112994 | $3894 | $7483\n15 | $948 | $108839 | $4155 | $7222\n16 | $948 | $104405 | $4433 | $6944\n17 | $948 | $99675 | $4730 | $6647\n18 | $948 | $94628 | $5047 | $6330\n19 | $948 | $89243 | $5385 | $5992\n20 | $948 | $83498 | $5746 | $5632\n21 | $948 | $77368 | $6130 | $5247\n22 | $948 | $70827 | $6541 | $4836\n23 | $948 | $63848 | $6979 | $4398\n24 | $948 | $56401 | $7446 | $3931\n25 | $948 | $48456 | $7945 | $3432\n26 | $948 | $39979 | $8477 | $2900\n27 | $948 | $30934 | $9045 | $2332\n28 | $948 | $21284 | $9651 | $1727\n29 | $948 | $10987 | $10297 | $1080\n30 | $948 | $0 | $10987 | $391",
							"title":"year | monthly payment | ending balance | yearly principal paid | yearly interest paid\n1 | $948 | $148323 | $1677 | $9701\n2 | $948 | $146535 | $1789 | $9588\n3 | $948 | $144626 | $1909 | $9469\n4 | $948 | $142589 | $2037 | $9341\n5 | $948 | $140416 | $2173 | $9204\n6 | $948 | $138098 | $2318 | $9059\n7 | $948 | $135624 | $2474 | $8904\n8 | $948 | $132985 | $2639 | $8738\n9 | $948 | $130169 | $2816 | $8561\n10 | $948 | $127164 | $3005 | $8373\n11 | $948 | $123958 | $3206 | $8171\n12 | $948 | $120538 | $3421 | $7957\n13 | $948 | $116888 | $3650 | $7727\n14 | $948 | $112994 | $3894 | $7483\n15 | $948 | $108839 | $4155 | $7222\n16 | $948 | $104405 | $4433 | $6944\n17 | $948 | $99675 | $4730 | $6647\n18 | $948 | $94628 | $5047 | $6330\n19 | $948 | $89243 | $5385 | $5992\n20 | $948 | $83498 | $5746 | $5632\n21 | $948 | $77368 | $6130 | $5247\n22 | $948 | $70827 | $6541 | $4836\n23 | $948 | $63848 | $6979 | $4398\n24 | $948 | $56401 | $7446 | $3931\n25 | $948 | $48456 | $7945 | $3432\n26 | $948 | $39979 | $8477 | $2900\n27 | $948 | $30934 | $9045 | $2332\n28 | $948 | $21284 | $9651 | $1727\n29 | $948 | $10987 | $10297 | $1080\n30 | $948 | $0 | $10987 | $391",
							"width":546,
							"height":1045,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"year | monthly payment | ending balance | yearly principal paid | yearly interest paid\n1 | $948 | $148323 | $1677 | $9701\n2 | $948 | $146535 | $1789 | $9588\n3 | $948 | $144626 | $1909 | $9469\n4 | $948 | $142589 | $2037 | $9341\n5 | $948 | $140416 | $2173 | $9204\n6 | $948 | $138098 | $2318 | $9059\n7 | $948 | $135624 | $2474 | $8904\n8 | $948 | $132985 | $2639 | $8738\n9 | $948 | $130169 | $2816 | $8561\n10 | $948 | $127164 | $3005 | $8373\n11 | $948 | $123958 | $3206 | $8171\n12 | $948 | $120538 | $3421 | $7957\n13 | $948 | $116888 | $3650 | $7727\n14 | $948 | $112994 | $3894 | $7483\n15 | $948 | $108839 | $4155 | $7222\n16 | $948 | $104405 | $4433 | $6944\n17 | $948 | $99675 | $4730 | $6647\n18 | $948 | $94628 | $5047 | $6330\n19 | $948 | $89243 | $5385 | $5992\n20 | $948 | $83498 | $5746 | $5632\n21 | $948 | $77368 | $6130 | $5247\n22 | $948 | $70827 | $6541 | $4836\n23 | $948 | $63848 | $6979 | $4398\n24 | $948 | $56401 | $7446 | $3931\n25 | $948 | $48456 | $7945 | $3432\n26 | $948 | $39979 | $8477 | $2900\n27 | $948 | $30934 | $9045 | $2332\n28 | $948 | $21284 | $9651 | $1727\n29 | $948 | $10987 | $10297 | $1080\n30 | $948 | $0 | $10987 | $391"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				}
			}
		],
		"assumptions":[
			{
				"type":"SelectFormulaVariable",
				"count":3,
				"word":"$150,000",
				"values":[
					{
						"name":"FixedRateMortgage",
						"desc":"loan amount",
						"input":"*FSVar.Q.*10.17-.%24150%2C000-_*FixedRateMortgage.MA-"
					},
					{
						"name":"FixedRateMortgage",
						"desc":"sale amount",
						"input":"*FSVar.Q.*10.17-.%24150%2C000-_*FixedRateMortgage.SaleAmountHouse-"
					},
					{
						"name":"FixedRateMortgage",
						"desc":"down payment",
						"input":"*FSVar.Q.*10.17-.%24150%2C000-_*FixedRateMortgage.DP-"
					}
				]
			},
			{
				"type":"SelectFormulaVariable",
				"count":2,
				"word":"6.5%",
				"values":[
					{
						"name":"FixedRateMortgage",
						"desc":"annual percentage rate",
						"input":"*FSVar.Q.*20.23-.6%21.5%25-_*FixedRateMortgage.AnnualPercentageRateFixed-"
					},
					{
						"name":"FixedRateMortgage",
						"desc":"down payment",
						"input":"*FSVar.Q.*20.23-.6%21.5%25-_*FixedRateMortgage.DP-"
					}
				]
			},
			{
				"type":"FormulaSelect",
				"template":"Assuming ${desc1}. Use ${desc2} instead",
				"count":3,
				"values":[
					{
						"name":"FixedRateMortgage",
						"desc":"fixed rate mortgage",
						"input":"FSelect_**FixedRateMortgage--"
					},
					{
						"name":"AdjustableRateMortgage",
						"desc":"adjustable rate mortgage",
						"input":"FSelect_**AdjustableRateMortgage--"
					},
					{
						"name":"RentVsBuy",
						"desc":"rent vs. buy",
						"input":"FSelect_**RentVsBuy--"
					}
				]
			},
			{
				"type":"FormulaVariableInclude",
				"template":"Also include ${desc1}",
				"count":4,
				"values":[
					{
						"name":"FixedRateMortgage.PTS",
						"desc":"points",
						"input":"*FVarOpt-_**FixedRateMortgage.PTS-.*FixedRateMortgage.MA--"
					},
					{
						"name":"FixedRateMortgage.IOP",
						"desc":"interest‐only period",
						"input":"*FVarOpt-_**FixedRateMortgage.IOP-.*FixedRateMortgage.MA--"
					},
					{
						"name":"FixedRateMortgage.MTR",
						"desc":"tax rate",
						"input":"*FVarOpt-_**FixedRateMortgage.MTR-.*FixedRateMortgage.MA--"
					},
					{
						"name":"FixedRateMortgage.BP",
						"desc":"balloon payment",
						"input":"*FVarOpt-_**FixedRateMortgage.BP-.*FixedRateMortgage.MA--"
					}
				]
			}
		]
	}
}"#
    ).unwrap();

    let h: RawApiResponse = serde_json::from_str(
r#"{
	"queryresult":{
		"success":true,
		"error":false,
		"numpods":9,
		"datatypes":"Country,Financial,WorldDevelopment",
		"timedout":"",
		"timedoutpods":"",
		"timing":3.636,
		"parsetiming":0.117,
		"parsetimedout":false,
		"recalculate":"",
		"id":"MSP1602126aa3h44g89278300006703b85ccd66hgd2",
		"host":"https:\/\/www6b3.wolframalpha.com",
		"server":"5",
		"related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa1603126aa3h44g89278300001af63205a220d3gh6235443652248981561",
		"version":"2.6",
		"inputstring":"france gdp",
		"pods":[
			{
				"title":"Input interpretation",
				"scanner":"Identity",
				"id":"Input",
				"position":100,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP1604126aa3h44g89278300005af19ecgii4720d9?MSPStoreType=image\/gif&s=5",
							"alt":"France | GDP | nominal",
							"title":"France | GDP | nominal",
							"width":214,
							"height":33,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"France | GDP | nominal"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				},
				"definitions":{
					"word":"GDP, nominal",
					"desc":"The sum of gross value added by all resident producers in the economy plus any product taxes and minus any subsidies not included in the value of the products. It is calculated without making deductions for depreciation of fabricated assets or for depletion and degradation of natural resources. Data is in current U.S. dollars. Dollar figures for GDP are converted from domestic currencies using single year official exchange rates. For a few countries where the official exchange rate does not reflect the rate effectively applied to actual foreign exchange transactions, an alternative conversion factor is used."
				}
			},
			{
				"title":"Result",
				"scanner":"Data",
				"id":"Result",
				"position":200,
				"error":false,
				"numsubpods":1,
				"primary":true,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":[
								"CountryData",
								"WorldDevelopmentData"
							]
						},
						"datasources":{
							"datasource":"WorldDevelopmentIndicatorsWorldBank"
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP1605126aa3h44g892783000053i5cef559g9de98?MSPStoreType=image\/gif&s=5",
							"alt":"$2.63 trillion per year (world rank: 6th) (2020 estimate)",
							"title":"$2.63 trillion per year (world rank: 6th) (2020 estimate)",
							"width":340,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"$2.63 trillion per year (world rank: 6th) (2020 estimate)"
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			},
			{
				"title":"Local currency conversion",
				"scanner":"Money",
				"id":"LocalCurrencyConversion",
				"position":300,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP1606126aa3h44g89278300004604cg5fci92ci36?MSPStoreType=image\/gif&s=5",
							"alt":"25.69 trillion kr per year (Swedish kronor per year) (^29\/_4-2022)",
							"title":"25.69 trillion kr per year (Swedish kronor per year) (^29\/_4-2022)",
							"width":402,
							"height":19,
							"type":"Default",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"25.69 trillion kr per year (Swedish kronor per year) (^29\/_4-2022)"
					}
				],
				"expressiontypes":{
					"name":"Default"
				}
			},
			{
				"title":"GDP history",
				"scanner":"Data",
				"id":"History:GDP:WorldDevelopmentData",
				"position":400,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"WorldDevelopmentData"
						},
						"datasources":{
							"datasource":"WorldDevelopmentIndicatorsWorldBank"
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP1607126aa3h44g89278300002d927ac6hbbf899i?MSPStoreType=image\/gif&s=5",
							"alt":"GDP history",
							"title":"",
							"width":546,
							"height":217,
							"type":"TimeSeriesPlot_1",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":""
					}
				],
				"expressiontypes":{
					"name":"Default"
				},
				"states":[
					{
						"name":"Log scale",
						"input":"History:GDP:WorldDevelopmentData__Log scale"
					},
					{
						"count":3,
						"value":"All years",
						"delimiters":"",
						"states":[
							{
								"name":"Last 5 years",
								"input":"History:GDP:WorldDevelopmentData__Last 5 years"
							},
							{
								"name":"Last 10 years",
								"input":"History:GDP:WorldDevelopmentData__Last 10 years"
							},
							{
								"name":"All years",
								"input":"History:GDP:WorldDevelopmentData__All years"
							}
						]
					}
				]
			},
			{
				"title":"Exchange history for $2.63 trillion (US dollars)",
				"scanner":"Money",
				"id":"History",
				"position":500,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"FinancialData"
						},
						"datasources":{
							"datasource":[
								"Morningstar",
								"NasdaqDataLink"
							]
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP1608126aa3h44g892783000043ch93ci81c7ff4i?MSPStoreType=image\/gif&s=5",
							"alt":"\n1-year minimum | 21.73 trillion kr (^10\/_6-2021 | 11 months ago)\n1-year maximum | 26.27 trillion kr (^8\/_3-2022 | 2 months ago)\n1-year average | 23.39 trillion kr (annualized volatility: 10%)",
							"title":"\n1-year minimum | 21.73 trillion kr (^10\/_6-2021 | 11 months ago)\n1-year maximum | 26.27 trillion kr (^8\/_3-2022 | 2 months ago)\n1-year average | 23.39 trillion kr (annualized volatility: 10%)",
							"width":450,
							"height":246,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"\n1-year minimum | 21.73 trillion kr (^10\/_6-2021 | 11 months ago)\n1-year maximum | 26.27 trillion kr (^8\/_3-2022 | 2 months ago)\n1-year average | 23.39 trillion kr (annualized volatility: 10%)"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				},
				"states":[
					{
						"count":4,
						"value":"Last year",
						"delimiters":"",
						"states":[
							{
								"name":"Last month",
								"input":"History__Last month"
							},
							{
								"name":"Last year",
								"input":"History__Last year"
							},
							{
								"name":"Last 5 years",
								"input":"History__Last 5 years"
							},
							{
								"name":"Last 10 years",
								"input":"History__Last 10 years"
							}
						]
					}
				],
				"infos":{
					"units":[
						{
							"short":"1.00 kr",
							"long":"Swedish kronor"
						},
						{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP1609126aa3h44g892783000059d7bg8dag21he6h?MSPStoreType=image\/gif&s=5",
							"width":"168",
							"height":"27"
						}
					]
				}
			},
			{
				"title":"Economic properties",
				"scanner":"Data",
				"id":"EconomicPropertiesForGDP:WorldDevelopmentData",
				"position":600,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"WorldDevelopmentData"
						},
						"datasources":{
							"datasource":"WorldDevelopmentIndicatorsWorldBank"
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP1610126aa3h44g892783000051984856b81a4he1?MSPStoreType=image\/gif&s=5",
							"alt":"GDP at exchange rate | $2.63 trillion per year (world rank: 6th)\nGDP at parity | $3.115 trillion per year (world rank: 9th)\nreal GDP | $2.41 trillion per year (price-adjusted to year-2000 US dollars) (world rank: 7th)\nGDP in local currency | €2.303 trillion per year\nGDP per capita | $39030 per year per person (world rank: 37th)\nGDP real growth | -7.855% per year (world rank: 175th)\nconsumer price inflation | +0.48% per year (world rank: 147th)\nunemployment rate | 8.62% (world rank: 71st highest)\n(2020 estimate)",
							"title":"GDP at exchange rate | $2.63 trillion per year (world rank: 6th)\nGDP at parity | $3.115 trillion per year (world rank: 9th)\nreal GDP | $2.41 trillion per year (price-adjusted to year-2000 US dollars) (world rank: 7th)\nGDP in local currency | €2.303 trillion per year\nGDP per capita | $39030 per year per person (world rank: 37th)\nGDP real growth | -7.855% per year (world rank: 175th)\nconsumer price inflation | +0.48% per year (world rank: 147th)\nunemployment rate | 8.62% (world rank: 71st highest)\n(2020 estimate)",
							"width":546,
							"height":311,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"GDP at exchange rate | $2.63 trillion per year (world rank: 6th)\nGDP at parity | $3.115 trillion per year (world rank: 9th)\nreal GDP | $2.41 trillion per year (price-adjusted to year-2000 US dollars) (world rank: 7th)\nGDP in local currency | €2.303 trillion per year\nGDP per capita | $39030 per year per person (world rank: 37th)\nGDP real growth | -7.855% per year (world rank: 175th)\nconsumer price inflation | +0.48% per year (world rank: 147th)\nunemployment rate | 8.62% (world rank: 71st highest)\n(2020 estimate)"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				},
				"infos":{
					"units":[
						{
							"short":"€per year",
							"long":"euros per year"
						},
						{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP1611126aa3h44g89278300000iia3ehee4i1ac13?MSPStoreType=image\/gif&s=5",
							"width":"181",
							"height":"27"
						}
					]
				},
				"definitions":[
					{
						"word":"GDP at parity",
						"desc":"PPP GDP is gross domestic product converted to international dollars using purchasing power parity rates. An international dollar has the same purchasing power over GDP as the U.S. dollar has in the United States. GDP is the sum of gross value added by all resident producers in the economy plus any product taxes and minus any subsidies not included in the value of the products. It is calculated without making deductions for depreciation of fabricated assets or for depletion and degradation of natural resources. Data are in current international dollars."
					},
					{
						"word":"real GDP",
						"desc":"The sum of gross value added by all resident producers in the economy plus any product taxes and minus any subsidies not included in the value of the products. It is calculated without making deductions for depreciation of fabricated assets or for depletion and degradation of natural resources. Data is in constant 2000 U.S. dollars. Dollar figures for GDP are converted from domestic currencies using 2000 official exchange rates. For a few countries where the official exchange rate does not reflect the rate effectively applied to actual foreign exchange transactions, an alternative conversion factor is used."
					},
					{
						"word":"GDP in local currency",
						"desc":"The sum of gross value added by all resident producers in the economy plus any product taxes and minus any subsidies not included in the value of the products. It is calculated without making deductions for depreciation of fabricated assets or for depletion and degradation of natural resources. Data is in current local currency."
					},
					{
						"word":"GDP per capita",
						"desc":"Gross domestic product divided by midyear population. GDP is the sum of gross value added by all resident producers in the economy plus any product taxes and minus any subsidies not included in the value of the products. It is calculated without making deductions for depreciation of fabricated assets or for depletion and degradation of natural resources. Data is in current U.S. dollars."
					},
					{
						"word":"GDP real growth",
						"desc":"The sum of gross value added by all resident producers in the economy plus any product taxes and minus any subsidies not included in the value of the products. It is calculated without making deductions for depreciation of fabricated assets or for depletion and degradation of natural resources. Aggregates are based on constant 2000 U.S. dollars."
					},
					{
						"word":"consumer price inflation",
						"desc":"The annual percentage change in the cost to the average consumer of acquiring a basket of goods and services that may be fixed or changed at specified intervals, such as yearly. The Laspeyres formula is generally used."
					}
				]
			},
			{
				"title":"GDP components",
				"scanner":"Data",
				"id":"GDPComponents:WorldDevelopmentData",
				"position":700,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"WorldDevelopmentData"
						},
						"datasources":{
							"datasource":"WorldDevelopmentIndicatorsWorldBank"
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP1612126aa3h44g89278300004546c2a4ighe38d2?MSPStoreType=image\/gif&s=5",
							"alt":"final consumption expenditure | $2.057 trillion per year (78.21%) (world rank: 6th)\ngross capital formation | $626.3 billion per year (23.81%) (world rank: 6th)\nexternal balance on goods and services | -$53.14 billion per year (-2.02%) (world rank: 203rd)\ntotal GDP | $2.63 trillion per year (100%) (world rank: 6th)\n(2020 estimate)",
							"title":"final consumption expenditure | $2.057 trillion per year (78.21%) (world rank: 6th)\ngross capital formation | $626.3 billion per year (23.81%) (world rank: 6th)\nexternal balance on goods and services | -$53.14 billion per year (-2.02%) (world rank: 203rd)\ntotal GDP | $2.63 trillion per year (100%) (world rank: 6th)\n(2020 estimate)",
							"width":546,
							"height":233,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"final consumption expenditure | $2.057 trillion per year (78.21%) (world rank: 6th)\ngross capital formation | $626.3 billion per year (23.81%) (world rank: 6th)\nexternal balance on goods and services | -$53.14 billion per year (-2.02%) (world rank: 203rd)\ntotal GDP | $2.63 trillion per year (100%) (world rank: 6th)\n(2020 estimate)"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				},
				"states":[
					{
						"name":"Show details",
						"input":"GDPComponents:WorldDevelopmentData__Show details"
					}
				],
				"definitions":[
					{
						"word":"total GDP",
						"desc":"The sum of gross value added by all resident producers in the economy plus any product taxes and minus any subsidies not included in the value of the products. It is calculated without making deductions for depreciation of fabricated assets or for depletion and degradation of natural resources. Data is in current U.S. dollars. Dollar figures for GDP are converted from domestic currencies using single year official exchange rates. For a few countries where the official exchange rate does not reflect the rate effectively applied to actual foreign exchange transactions, an alternative conversion factor is used."
					},
					{
						"word":"final consumption expenditure",
						"desc":"The sum of household final consumption expenditure (private consumption) and general government final consumption expenditure (general government consumption). Data is in current U.S. dollars."
					},
					{
						"word":"gross capital formation",
						"desc":"Outlays on additions to the fixed assets of the economy plus net changes in the level of inventories. Fixed assets include land improvements (fences, ditches, drains, etc.); plant, machinery, and equipment purchases; and the construction of roads, railways, and the like, including schools, offices, hospitals, private residential dwellings, and commercial and industrial buildings. Inventories are stocks of goods held by firms to meet temporary or unexpected fluctuations in production or sales, and \"work in progress.\" According to the 1993 SNA, net acquisitions of valuables are also considered capital formation. Data is in current U.S. dollars."
					},
					{
						"word":"external balance on goods and services",
						"desc":"Exports of goods and services minus imports of goods and services (previously nonfactor services). Data is in current U.S. dollars."
					}
				]
			},
			{
				"title":"Value added by sector",
				"scanner":"Data",
				"id":"ValueAdded:WorldDevelopmentData",
				"position":800,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"WorldDevelopmentData"
						},
						"datasources":{
							"datasource":"WorldDevelopmentIndicatorsWorldBank"
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP1613126aa3h44g89278300003ehhgbe13b9h53g2?MSPStoreType=image\/gif&s=5",
							"alt":"agriculture | $42 billion per year (world rank: 12th) (2020 estimate)\nindustry | $432.6 billion per year (world rank: 9th) (2020 estimate)\nmanufacturing | $247 billion per year (world rank: 8th) (2020 estimate)\nservices, etc. | $1.737 trillion per year (world rank: 6th) (2016 estimate)",
							"title":"agriculture | $42 billion per year (world rank: 12th) (2020 estimate)\nindustry | $432.6 billion per year (world rank: 9th) (2020 estimate)\nmanufacturing | $247 billion per year (world rank: 8th) (2020 estimate)\nservices, etc. | $1.737 trillion per year (world rank: 6th) (2016 estimate)",
							"width":496,
							"height":136,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"agriculture | $42 billion per year (world rank: 12th) (2020 estimate)\nindustry | $432.6 billion per year (world rank: 9th) (2020 estimate)\nmanufacturing | $247 billion per year (world rank: 8th) (2020 estimate)\nservices, etc. | $1.737 trillion per year (world rank: 6th) (2016 estimate)"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				},
				"states":[
					{
						"name":"Show manufacturing breakdown",
						"input":"ValueAdded:WorldDevelopmentData__Show manufacturing breakdown"
					}
				],
				"definitions":[
					{
						"word":"agriculture",
						"desc":"Corresponds to ISIC divisions 1-5 and includes forestry, hunting, and fishing, as well as cultivation of crops and livestock production. Value added is the net output of a sector after adding up all outputs and subtracting intermediate inputs. It is calculated without making deductions for depreciation of fabricated assets or depletion and degradation of natural resources. The origin of value added is determined by the International Standard Industrial Classification (ISIC), revision 3. Data is in current U.S. dollars."
					},
					{
						"word":"industry",
						"desc":"Corresponds to ISIC divisions 10-45 and includes manufacturing (ISIC divisions 15-37). It comprises value added in mining, manufacturing (also reported as a separate subgroup), construction, electricity, water, and gas. Value added is the net output of a sector after adding up all outputs and subtracting intermediate inputs. It is calculated without making deductions for depreciation of fabricated assets or depletion and degradation of natural resources. The origin of value added is determined by the International Standard Industrial Classification (ISIC), revision 3. Data is in current U.S. dollars."
					},
					{
						"word":"manufacturing",
						"desc":"Refers to industries belonging to ISIC divisions 15-37. Value added is the net output of a sector after adding up all outputs and subtracting intermediate inputs. It is calculated without making deductions for depreciation of fabricated assets or depletion and degradation of natural resources. The origin of value added is determined by the International Standard Industrial Classification (ISIC), revision 3. Data is in current U.S. dollars."
					},
					{
						"word":"services, etc.",
						"desc":"Correspond to ISIC divisions 50-99. They include value added in wholesale and retail trade (including hotels and restaurants), transport, and government, financial, professional, and personal services such as education, health care, and real estate services. Also included are imputed bank service charges, import duties, and any statistical discrepancies noted by national compilers as well as discrepancies arising from rescaling. Value added is the net output of a sector after adding up all outputs and subtracting intermediate inputs. It is calculated without making deductions for depreciation of fabricated assets or depletion and degradation of natural resources. The industrial origin of value added is determined by the International Standard Industrial Classification (ISIC), revision 3. Data is in current U.S. dollars."
					}
				]
			},
			{
				"title":"Additional currency conversions for $2.63 trillion (US dollars)",
				"scanner":"Money",
				"id":"CurrencyConversions",
				"position":900,
				"error":false,
				"numsubpods":1,
				"subpods":[
					{
						"title":"",
						"microsources":{
							"microsource":"FinancialData"
						},
						"datasources":{
							"datasource":[
								"Morningstar",
								"NasdaqDataLink"
							]
						},
						"img":{
							"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP1614126aa3h44g89278300004ih1ii05c63b5e4f?MSPStoreType=image\/gif&s=5",
							"alt":"USD | $2.63 trillion per year (US dollars per year)\nEUR | €2.496 trillion per year (euros per year)\nCNY | ￥17.33 trillion per year (Chinese yuan per year)\nJPY | ¥341.9 trillion per year (Japanese yen per year)\nGBP | £2.094 trillion per year (British pounds per year)\nCAD | C$3.351 trillion per year (Canadian dollars per year)\nMXN | $53.81 trillion per year (Mexican pesos per year)\nGTQ | Q20.15 trillion per year (Guatemala quetzales per year)\nCRC | ₡1.74 quadrillion per year (Costa Rican colones per year)\nHNL | L64.48 trillion per year (Honduran lempiras per year)\nJMD | J$406.9 trillion per year (Jamaican dollars per year)\nXAU | 47545.46 tn\/yr (gold tons per year)\nXAG | 3.965 million tn\/yr (silver tons per year)",
							"title":"USD | $2.63 trillion per year (US dollars per year)\nEUR | €2.496 trillion per year (euros per year)\nCNY | ￥17.33 trillion per year (Chinese yuan per year)\nJPY | ¥341.9 trillion per year (Japanese yen per year)\nGBP | £2.094 trillion per year (British pounds per year)\nCAD | C$3.351 trillion per year (Canadian dollars per year)\nMXN | $53.81 trillion per year (Mexican pesos per year)\nGTQ | Q20.15 trillion per year (Guatemala quetzales per year)\nCRC | ₡1.74 quadrillion per year (Costa Rican colones per year)\nHNL | L64.48 trillion per year (Honduran lempiras per year)\nJMD | J$406.9 trillion per year (Jamaican dollars per year)\nXAU | 47545.46 tn\/yr (gold tons per year)\nXAG | 3.965 million tn\/yr (silver tons per year)",
							"width":465,
							"height":382,
							"type":"Grid",
							"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
							"colorinvertable":true,
							"contenttype":"image\/gif"
						},
						"plaintext":"USD | $2.63 trillion per year (US dollars per year)\nEUR | €2.496 trillion per year (euros per year)\nCNY | ￥17.33 trillion per year (Chinese yuan per year)\nJPY | ¥341.9 trillion per year (Japanese yen per year)\nGBP | £2.094 trillion per year (British pounds per year)\nCAD | C$3.351 trillion per year (Canadian dollars per year)\nMXN | $53.81 trillion per year (Mexican pesos per year)\nGTQ | Q20.15 trillion per year (Guatemala quetzales per year)\nCRC | ₡1.74 quadrillion per year (Costa Rican colones per year)\nHNL | L64.48 trillion per year (Honduran lempiras per year)\nJMD | J$406.9 trillion per year (Jamaican dollars per year)\nXAU | 47545.46 tn\/yr (gold tons per year)\nXAG | 3.965 million tn\/yr (silver tons per year)"
					}
				],
				"expressiontypes":{
					"name":"Grid"
				},
				"states":[
					{
						"name":"World currencies",
						"input":"CurrencyConversions__World currencies"
					}
				]
			}
		],
		"sources":[
			{
				"url":"https:\/\/www6b3.wolframalpha.com\/sources\/CountryDataSourceInformationNotes.html",
				"text":"Country data"
			},
			{
				"url":"https:\/\/www6b3.wolframalpha.com\/sources\/FinancialDataSourceInformationNotes.html",
				"text":"Financial data"
			},
			{
				"url":"https:\/\/www6b3.wolframalpha.com\/sources\/WorldDevelopmentDataSourceInformationNotes.html",
				"text":"World development data"
			}
		]
	}
}"#
    ).unwrap();

    let i: RawApiResponse = serde_json::from_str(
        r#"{
            "queryresult":{
                "success":true,
                "error":false,
                "numpods":7,
                "datatypes":"Aircraft",
                "timedout":"",
                "timedoutpods":"",
                "timing":1.242,
                "parsetiming":0.435,
                "parsetimedout":false,
                "recalculate":"",
                "id":"MSP875516cbd23e29bg8i560000148ei2526f0f2e7c",
                "host":"https:\/\/www6b3.wolframalpha.com",
                "server":"20",
                "related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa875616cbd23e29bg8i560000238f7e5ifbde26e38458888475783217078",
                "version":"2.6",
                "inputstring":"F\/A-18E\/F Super Hornet",
                "pods":[
                    {
                        "title":"Input interpretation",
                        "scanner":"Identity",
                        "id":"Input",
                        "position":100,
                        "error":false,
                        "numsubpods":1,
                        "subpods":[
                            {
                                "title":"",
                                "img":{
                                    "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP875716cbd23e29bg8i56000039didebah5b42f49?MSPStoreType=image\/gif&s=20",
                                    "alt":"F\/A-18E\/F Super Hornet (aircraft)",
                                    "title":"F\/A-18E\/F Super Hornet (aircraft)",
                                    "width":231,
                                    "height":19,
                                    "type":"Default",
                                    "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                    "colorinvertable":true,
                                    "contenttype":"image\/gif"
                                },
                                "plaintext":"F\/A-18E\/F Super Hornet (aircraft)"
                            }
                        ],
                        "expressiontypes":{
                            "name":"Default"
                        }
                    },
                    {
                        "title":"Basic information",
                        "scanner":"Data",
                        "id":"Basic:AircraftData",
                        "position":200,
                        "error":false,
                        "numsubpods":1,
                        "subpods":[
                            {
                                "title":"",
                                "microsources":{
                                    "microsource":"AircraftData"
                                },
                                "img":{
                                    "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP875816cbd23e29bg8i5600001cc8a9ff21hbi9if?MSPStoreType=image\/gif&s=20",
                                    "alt":"type | multirole fighter, strike fighter\nmanufacturer | Boeing Integrated Defense Systems",
                                    "title":"type | multirole fighter, strike fighter\nmanufacturer | Boeing Integrated Defense Systems",
                                    "width":368,
                                    "height":70,
                                    "type":"Grid",
                                    "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                    "colorinvertable":true,
                                    "contenttype":"image\/gif"
                                },
                                "plaintext":"type | multirole fighter, strike fighter\nmanufacturer | Boeing Integrated Defense Systems"
                            }
                        ],
                        "expressiontypes":{
                            "name":"Grid"
                        }
                    },
                    {
                        "title":"Image",
                        "scanner":"Data",
                        "id":"Image:AircraftData",
                        "position":300,
                        "error":false,
                        "numsubpods":1,
                        "subpods":[
                            {
                                "title":"",
                                "imagesource":"http:\/\/en.wikipedia.org\/wiki\/File:FA-18_Hornet_VFA-41.jpg",
                                "microsources":{
                                    "microsource":"AircraftData"
                                },
                                "img":{
                                    "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP875916cbd23e29bg8i560000460fa7g7d5b76gd2?MSPStoreType=image\/gif&s=20",
                                    "alt":"Image",
                                    "title":"",
                                    "width":150,
                                    "height":107,
                                    "type":"Default",
                                    "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                    "colorinvertable":false,
                                    "contenttype":"image\/gif"
                                },
                                "plaintext":""
                            }
                        ],
                        "expressiontypes":{
                            "name":"Default"
                        }
                    },
                    {
                        "title":"General characteristics",
                        "scanner":"Data",
                        "id":"GeneralCharacteristics:AircraftData",
                        "position":400,
                        "error":false,
                        "numsubpods":1,
                        "subpods":[
                            {
                                "title":"",
                                "microsources":{
                                    "microsource":"AircraftData"
                                },
                                "img":{
                                    "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP876016cbd23e29bg8i560000529348082ba4748c?MSPStoreType=image\/gif&s=20",
                                    "alt":"crew | 2 people\nheight | 4.88 meters",
                                    "title":"crew | 2 people\nheight | 4.88 meters",
                                    "width":165,
                                    "height":70,
                                    "type":"Grid",
                                    "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                    "colorinvertable":true,
                                    "contenttype":"image\/gif"
                                },
                                "plaintext":"crew | 2 people\nheight | 4.88 meters"
                            }
                        ],
                        "expressiontypes":{
                            "name":"Grid"
                        },
                        "states":[
                            {
                                "name":"Show non-metric",
                                "input":"GeneralCharacteristics:AircraftData__Show non-metric"
                            },
                            {
                                "name":"More",
                                "input":"GeneralCharacteristics:AircraftData__More"
                            }
                        ]
                    },
                    {
                        "title":"Performance",
                        "scanner":"Data",
                        "id":"Performance:AircraftData",
                        "position":500,
                        "error":false,
                        "numsubpods":1,
                        "subpods":[
                            {
                                "title":"",
                                "microsources":{
                                    "microsource":"AircraftData"
                                },
                                "img":{
                                    "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP876116cbd23e29bg8i560000559aac5ah7g0522d?MSPStoreType=image\/gif&s=20",
                                    "alt":"range with maximum load | 2361 km (kilometers)\n1275 nmi (nautical miles)\nmaximum weight | 29900 kg (kilograms)\n33 sh tn (short tons)",
                                    "title":"range with maximum load | 2361 km (kilometers)\n1275 nmi (nautical miles)\nmaximum weight | 29900 kg (kilograms)\n33 sh tn (short tons)",
                                    "width":391,
                                    "height":104,
                                    "type":"Grid",
                                    "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                    "colorinvertable":true,
                                    "contenttype":"image\/gif"
                                },
                                "plaintext":"range with maximum load | 2361 km (kilometers)\n1275 nmi (nautical miles)\nmaximum weight | 29900 kg (kilograms)\n33 sh tn (short tons)"
                            }
                        ],
                        "expressiontypes":{
                            "name":"Grid"
                        },
                        "states":[
                            {
                                "name":"Show non-metric",
                                "input":"Performance:AircraftData__Show non-metric"
                            },
                            {
                                "name":"More",
                                "input":"Performance:AircraftData__More"
                            }
                        ]
                    },
                    {
                        "title":"History",
                        "scanner":"Data",
                        "id":"History:AircraftData",
                        "position":600,
                        "error":false,
                        "numsubpods":1,
                        "subpods":[
                            {
                                "title":"",
                                "microsources":{
                                    "microsource":"AircraftData"
                                },
                                "img":{
                                    "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP876216cbd23e29bg8i5600003i14h6be83a350bh?MSPStoreType=image\/gif&s=20",
                                    "alt":"name of designer | McDonnell Douglas\nnumber of aircraft built | 300\ndate of first flight | Wednesday, November 29, 1995 (26 years ago)",
                                    "title":"name of designer | McDonnell Douglas\nnumber of aircraft built | 300\ndate of first flight | Wednesday, November 29, 1995 (26 years ago)",
                                    "width":499,
                                    "height":103,
                                    "type":"Grid",
                                    "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                    "colorinvertable":true,
                                    "contenttype":"image\/gif"
                                },
                                "plaintext":"name of designer | McDonnell Douglas\nnumber of aircraft built | 300\ndate of first flight | Wednesday, November 29, 1995 (26 years ago)"
                            }
                        ],
                        "expressiontypes":{
                            "name":"Grid"
                        }
                    },
                    {
                        "title":"Wikipedia summary",
                        "scanner":"Data",
                        "id":"WikipediaSummary:AircraftData",
                        "position":700,
                        "error":false,
                        "numsubpods":1,
                        "subpods":[
                            {
                                "title":"",
                                "img":{
                                    "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP876316cbd23e29bg8i560000153fd12hgchdac31?MSPStoreType=image\/gif&s=20",
                                    "alt":"The Boeing F\/A-18E and F\/A-18F Super Hornet are twin-engine, carrier-capable, multirole fighter aircraft variants based on the McDonnell Douglas F\/A-18 Hornet. The F\/A-18E single-seat and F\/A-18F tandem-seat variants are larger and more advanced derivatives of the F\/A-18C and D Hornet.",
                                    "title":"The Boeing F\/A-18E and F\/A-18F Super Hornet are twin-engine, carrier-capable, multirole fighter aircraft variants based on the McDonnell Douglas F\/A-18 Hornet. The F\/A-18E single-seat and F\/A-18F tandem-seat variants are larger and more advanced derivatives of the F\/A-18C and D Hornet.",
                                    "width":531,
                                    "height":80,
                                    "type":"Default",
                                    "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                    "colorinvertable":true,
                                    "contenttype":"image\/gif"
                                },
                                "plaintext":"The Boeing F\/A-18E and F\/A-18F Super Hornet are twin-engine, carrier-capable, multirole fighter aircraft variants based on the McDonnell Douglas F\/A-18 Hornet. The F\/A-18E single-seat and F\/A-18F tandem-seat variants are larger and more advanced derivatives of the F\/A-18C and D Hornet.",
                                "infos":{
                                    "links":{
                                        "url":"http:\/\/en.wikipedia.org\/wiki\/F\/A-18E\/F_Super_Hornet",
                                        "text":"Full entry"
                                    }
                                }
                            }
                        ],
                        "expressiontypes":{
                            "name":"Default"
                        }
                    }
                ],
                "sources":{
                    "url":"https:\/\/www6b3.wolframalpha.com\/sources\/AircraftDataSourceInformationNotes.html",
                    "text":"Aircraft data"
                }
            }
        }"#).unwrap();

    let j: RawApiResponse = serde_json::from_str(
            r#"{
                "queryresult":{
                    "success":true,
                    "error":false,
                    "numpods":4,
                    "datatypes":"Country,InternationalTransportation",
                    "timedout":"",
                    "timedoutpods":"",
                    "timing":2.066,
                    "parsetiming":1.091,
                    "parsetimedout":false,
                    "recalculate":"",
                    "id":"MSP653223b3026c193d53hd00005eh25e39b775if6a",
                    "host":"https:\/\/www6b3.wolframalpha.com",
                    "server":"14",
                    "related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa653323b3026c193d53hd00003h0d3h4dhad1ceg04307842769377011871",
                    "version":"2.6",
                    "inputstring":"autos in use per road length in the UK",
                    "pods":[
                        {
                            "title":"Input interpretation",
                            "scanner":"Identity",
                            "id":"Input",
                            "position":100,
                            "error":false,
                            "numsubpods":1,
                            "subpods":[
                                {
                                    "title":"",
                                    "img":{
                                        "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP653423b3026c193d53hd00002df5e85dega36dbg?MSPStoreType=image\/gif&s=14",
                                        "alt":"United Kingdom | total vehicles in use per road length",
                                        "title":"United Kingdom | total vehicles in use per road length",
                                        "width":369,
                                        "height":23,
                                        "type":"Grid",
                                        "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                        "colorinvertable":true,
                                        "contenttype":"image\/gif"
                                    },
                                    "plaintext":"United Kingdom | total vehicles in use per road length"
                                }
                            ],
                            "expressiontypes":{
                                "name":"Grid"
                            }
                        },
                        {
                            "title":"Result",
                            "scanner":"Data",
                            "id":"Result",
                            "position":200,
                            "error":false,
                            "numsubpods":1,
                            "primary":true,
                            "subpods":[
                                {
                                    "title":"",
                                    "microsources":{
                                        "microsource":[
                                            "CountryData",
                                            "InternationalTransportationData"
                                        ]
                                    },
                                    "datasources":{
                                        "datasource":[
                                            "InternationalRoadFederation",
                                            "ResearchAndInnovativeTechnologyAdministrationBureauOfTransportationStatistics",
                                            "USDepartmentOfTransportationFederalHighwayAdministration",
                                            "EurostatTransportationStatistics"
                                        ]
                                    },
                                    "img":{
                                        "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP653523b3026c193d53hd00001220dh4hgd658aa8?MSPStoreType=image\/gif&s=14",
                                        "alt":"80 vehicles per kilometer (2005 estimate)",
                                        "title":"80 vehicles per kilometer (2005 estimate)",
                                        "width":264,
                                        "height":19,
                                        "type":"Default",
                                        "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                        "colorinvertable":true,
                                        "contenttype":"image\/gif"
                                    },
                                    "plaintext":"80 vehicles per kilometer (2005 estimate)"
                                }
                            ],
                            "expressiontypes":{
                                "name":"Default"
                            },
                            "states":[
                                {
                                    "name":"Show non-metric",
                                    "input":"Result__Show non-metric"
                                }
                            ]
                        },
                        {
                            "title":"History",
                            "scanner":"Data",
                            "id":"History:VehiclesInUseTotalPerKilometersOfRoads:InternationalTransportationData",
                            "position":300,
                            "error":false,
                            "numsubpods":1,
                            "subpods":[
                                {
                                    "title":"",
                                    "microsources":{
                                        "microsource":"InternationalTransportationData"
                                    },
                                    "datasources":{
                                        "datasource":[
                                            "InternationalRoadFederation",
                                            "ResearchAndInnovativeTechnologyAdministrationBureauOfTransportationStatistics",
                                            "USDepartmentOfTransportationFederalHighwayAdministration",
                                            "EurostatTransportationStatistics"
                                        ]
                                    },
                                    "img":{
                                        "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP653623b3026c193d53hd0000691gb3add3279he8?MSPStoreType=image\/gif&s=14",
                                        "alt":"History",
                                        "title":"",
                                        "width":408,
                                        "height":160,
                                        "type":"TimeSeriesPlot_1",
                                        "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                        "colorinvertable":true,
                                        "contenttype":"image\/gif"
                                    },
                                    "plaintext":""
                                }
                            ],
                            "expressiontypes":{
                                "name":"Default"
                            }
                        },
                        {
                            "title":"Vehicles in use",
                            "scanner":"Data",
                            "id":"VehiclesInUse:InternationalTransportationData",
                            "position":400,
                            "error":false,
                            "numsubpods":1,
                            "subpods":[
                                {
                                    "title":"",
                                    "microsources":{
                                        "microsource":"InternationalTransportationData"
                                    },
                                    "datasources":{
                                        "datasource":[
                                            "InternationalRoadFederation",
                                            "ResearchAndInnovativeTechnologyAdministrationBureauOfTransportationStatistics",
                                            "USDepartmentOfTransportationFederalHighwayAdministration",
                                            "EurostatTransportationStatistics"
                                        ]
                                    },
                                    "img":{
                                        "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP653723b3026c193d53hd000068173hf37fb1b776?MSPStoreType=image\/gif&s=14",
                                        "alt":"passenger cars | 27.5 million cars (2005 estimate)\ntrucks and vans | 3.4 million vehicles (2005 estimate)\nbuses | 178000 buses (2005 estimate)\nmotorcycles | 1.2 million motorcycles (2005 estimate)\ntotal | 32.3 million vehicles (2005 estimate)",
                                        "title":"passenger cars | 27.5 million cars (2005 estimate)\ntrucks and vans | 3.4 million vehicles (2005 estimate)\nbuses | 178000 buses (2005 estimate)\nmotorcycles | 1.2 million motorcycles (2005 estimate)\ntotal | 32.3 million vehicles (2005 estimate)",
                                        "width":401,
                                        "height":169,
                                        "type":"Grid",
                                        "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                        "colorinvertable":true,
                                        "contenttype":"image\/gif"
                                    },
                                    "plaintext":"passenger cars | 27.5 million cars (2005 estimate)\ntrucks and vans | 3.4 million vehicles (2005 estimate)\nbuses | 178000 buses (2005 estimate)\nmotorcycles | 1.2 million motorcycles (2005 estimate)\ntotal | 32.3 million vehicles (2005 estimate)"
                                }
                            ],
                            "expressiontypes":{
                                "name":"Grid"
                            },
                            "states":[
                                {
                                    "count":3,
                                    "value":"Total number",
                                    "delimiters":"",
                                    "states":[
                                        {
                                            "name":"Total number",
                                            "input":"VehiclesInUse:InternationalTransportationData__Total number"
                                        },
                                        {
                                            "name":"Per population",
                                            "input":"VehiclesInUse:InternationalTransportationData__Per population"
                                        },
                                        {
                                            "name":"Per road length",
                                            "input":"VehiclesInUse:InternationalTransportationData__Per road length"
                                        }
                                    ]
                                }
                            ]
                        }
                    ],
                    "sources":[
                        {
                            "url":"https:\/\/www6b3.wolframalpha.com\/sources\/CountryDataSourceInformationNotes.html",
                            "text":"Country data"
                        },
                        {
                            "url":"https:\/\/www6b3.wolframalpha.com\/sources\/InternationalTransportationDataSourceInformationNotes.html",
                            "text":"International transportation data"
                        }
                    ]
                }
            }"#).unwrap();

    let k: RawApiResponse = serde_json::from_str(r#"{
                "queryresult":{
                    "success":true,
                    "error":false,
                    "numpods":7,
                    "datatypes":"ExpandedFood,Formula",
                    "timedout":"",
                    "timedoutpods":"",
                    "timing":2.054,
                    "parsetiming":0.638,
                    "parsetimedout":false,
                    "recalculate":"",
                    "id":"MSP43216cbdah0781aeec10000165aa88ha5i33ha4",
                    "host":"https:\/\/www6b3.wolframalpha.com",
                    "server":"20",
                    "related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa43316cbdah0781aeec100002218685e3g26g7bd8458888475783217078",
                    "version":"2.6",
                    "inputstring":"how many people does a 20 pound turkey feed",
                    "pods":[
                        {
                            "title":"Input information",
                            "scanner":"Formula",
                            "id":"Input",
                            "position":100,
                            "error":false,
                            "numsubpods":1,
                            "subpods":[
                                {
                                    "title":"",
                                    "img":{
                                        "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP43416cbdah0781aeec100001i311881ahe08gi7?MSPStoreType=image\/gif&s=20",
                                        "alt":"turkey portion counter | \nturkey weight | 20 lb (pounds)\nmeal proportions | light",
                                        "title":"turkey portion counter | \nturkey weight | 20 lb (pounds)\nmeal proportions | light",
                                        "width":252,
                                        "height":103,
                                        "type":"Grid",
                                        "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                        "colorinvertable":true,
                                        "contenttype":"image\/gif"
                                    },
                                    "plaintext":"turkey portion counter | \nturkey weight | 20 lb (pounds)\nmeal proportions | light"
                                }
                            ],
                            "expressiontypes":{
                                "name":"Grid"
                            }
                        },
                        {
                            "title":"Result",
                            "scanner":"Formula",
                            "id":"Result",
                            "position":200,
                            "error":false,
                            "numsubpods":1,
                            "primary":true,
                            "subpods":[
                                {
                                    "title":"",
                                    "primary":true,
                                    "img":{
                                        "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP43516cbdah0781aeec100005eed5106f32058h2?MSPStoreType=image\/gif&s=20",
                                        "alt":"number of adults | 20",
                                        "title":"number of adults | 20",
                                        "width":178,
                                        "height":37,
                                        "type":"Grid",
                                        "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                        "colorinvertable":true,
                                        "contenttype":"image\/gif"
                                    },
                                    "plaintext":"number of adults | 20"
                                }
                            ],
                            "expressiontypes":{
                                "name":"Grid"
                            }
                        },
                        {
                            "title":"Equation",
                            "scanner":"Formula",
                            "id":"Equation",
                            "position":300,
                            "error":false,
                            "numsubpods":1,
                            "subpods":[
                                {
                                    "title":"",
                                    "img":{
                                        "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP43616cbdah0781aeec1000011a75e88ifg59268?MSPStoreType=image\/gif&s=20",
                                        "alt":"W = A P 1 lb | | \nA | number of adults\nW | turkey weight\nP | meal proportions\nlb | pound (≈ 0.4536 kg)",
                                        "title":"W = A P 1 lb | | \nA | number of adults\nW | turkey weight\nP | meal proportions\nlb | pound (≈ 0.4536 kg)",
                                        "width":194,
                                        "height":170,
                                        "type":"Grid",
                                        "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                        "colorinvertable":true,
                                        "contenttype":"image\/gif"
                                    },
                                    "plaintext":"W = A P 1 lb | | \nA | number of adults\nW | turkey weight\nP | meal proportions\nlb | pound (≈ 0.4536 kg)"
                                }
                            ],
                            "expressiontypes":{
                                "name":"Grid"
                            }
                        },
                        {
                            "title":"Thawing times",
                            "scanner":"Formula",
                            "id":"ThawingTimes",
                            "position":400,
                            "error":false,
                            "numsubpods":1,
                            "subpods":[
                                {
                                    "title":"",
                                    "img":{
                                        "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP43716cbdah0781aeec10000621g1f2eee9e53b6?MSPStoreType=image\/gif&s=20",
                                        "alt":"thawing time in a refrigerator | 4 days\nthawing time in a cold water bath | 10 hours",
                                        "title":"thawing time in a refrigerator | 4 days\nthawing time in a cold water bath | 10 hours",
                                        "width":327,
                                        "height":70,
                                        "type":"Grid",
                                        "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                        "colorinvertable":true,
                                        "contenttype":"image\/gif"
                                    },
                                    "plaintext":"thawing time in a refrigerator | 4 days\nthawing time in a cold water bath | 10 hours"
                                }
                            ],
                            "expressiontypes":{
                                "name":"Grid"
                            }
                        },
                        {
                            "title":"Available meat",
                            "scanner":"Formula",
                            "id":"AvailableMeat",
                            "position":500,
                            "error":false,
                            "numsubpods":1,
                            "subpods":[
                                {
                                    "title":"",
                                    "img":{
                                        "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP43816cbdah0781aeec10000315deg342hfggeh0?MSPStoreType=image\/gif&s=20",
                                        "alt":"8 lb (pounds)\n(assuming 40% of total weight is usable meat)",
                                        "title":"8 lb (pounds)\n(assuming 40% of total weight is usable meat)",
                                        "width":266,
                                        "height":42,
                                        "type":"Default",
                                        "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                        "colorinvertable":true,
                                        "contenttype":"image\/gif"
                                    },
                                    "plaintext":"8 lb (pounds)\n(assuming 40% of total weight is usable meat)"
                                }
                            ],
                            "expressiontypes":{
                                "name":"Default"
                            },
                            "states":[
                                {
                                    "name":"Show metric",
                                    "input":"AvailableMeat__Show metric"
                                }
                            ]
                        },
                        {
                            "title":"Cooking time",
                            "scanner":"Formula",
                            "id":"CookingTime",
                            "position":600,
                            "error":false,
                            "numsubpods":1,
                            "subpods":[
                                {
                                    "title":"",
                                    "img":{
                                        "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP43916cbdah0781aeec100004641bigh5570h842?MSPStoreType=image\/gif&s=20",
                                        "alt":"4.5 hours\n(using the heat equation for a spherical turkey in a 325 °F oven at sea level)",
                                        "title":"4.5 hours\n(using the heat equation for a spherical turkey in a 325 °F oven at sea level)",
                                        "width":435,
                                        "height":43,
                                        "type":"Default",
                                        "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                        "colorinvertable":true,
                                        "contenttype":"image\/gif"
                                    },
                                    "plaintext":"4.5 hours\n(using the heat equation for a spherical turkey in a 325 °F oven at sea level)"
                                }
                            ],
                            "expressiontypes":{
                                "name":"Default"
                            }
                        },
                        {
                            "title":"Nutrition facts per adult portion",
                            "scanner":"Formula",
                            "id":"AdultPortions",
                            "position":700,
                            "error":false,
                            "numsubpods":1,
                            "subpods":[
                                {
                                    "title":"",
                                    "microsources":{
                                        "microsource":"ExpandedFoodData"
                                    },
                                    "img":{
                                        "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP44016cbdah0781aeec1000052f97g1i4h5ib890?MSPStoreType=image\/gif&s=20",
                                        "alt":"serving size 0.1814 kg (181 g)\ntotal calories 364 | fat calories 147\n% daily value^* | \n total fat 16 g | 25%\n saturated fat 5 g | 24%\n trans fat | \n cholesterol 158 mg | 53%\n sodium 124 mg | 5%\n total carbohydrates 0 g | 0%\n dietary fiber 0 g | 0%\n sugar 0 g | \n protein 51 g | 102%\n calcium 5% | iron 19% \n vitamin E 3% | thiamin 7% \n riboflavin 19% | niacin 45% \n vitamin B6 37% | vitamin B12 11% \n folate 3% | phosphorus 37% \n magnesium 11% | zinc 37% \n*percent daily values are based on a 2000 calorie diet\n(averaged over different types of turkey, roasted, with skin)",
                                        "title":"serving size 0.1814 kg (181 g)\ntotal calories 364 | fat calories 147\n% daily value^* | \n total fat 16 g | 25%\n saturated fat 5 g | 24%\n trans fat | \n cholesterol 158 mg | 53%\n sodium 124 mg | 5%\n total carbohydrates 0 g | 0%\n dietary fiber 0 g | 0%\n sugar 0 g | \n protein 51 g | 102%\n calcium 5% | iron 19% \n vitamin E 3% | thiamin 7% \n riboflavin 19% | niacin 45% \n vitamin B6 37% | vitamin B12 11% \n folate 3% | phosphorus 37% \n magnesium 11% | zinc 37% \n*percent daily values are based on a 2000 calorie diet\n(averaged over different types of turkey, roasted, with skin)",
                                        "width":349,
                                        "height":495,
                                        "type":"Default",
                                        "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                        "colorinvertable":true,
                                        "contenttype":"image\/gif"
                                    },
                                    "plaintext":"serving size 0.1814 kg (181 g)\ntotal calories 364 | fat calories 147\n% daily value^* | \n total fat 16 g | 25%\n saturated fat 5 g | 24%\n trans fat | \n cholesterol 158 mg | 53%\n sodium 124 mg | 5%\n total carbohydrates 0 g | 0%\n dietary fiber 0 g | 0%\n sugar 0 g | \n protein 51 g | 102%\n calcium 5% | iron 19% \n vitamin E 3% | thiamin 7% \n riboflavin 19% | niacin 45% \n vitamin B6 37% | vitamin B12 11% \n folate 3% | phosphorus 37% \n magnesium 11% | zinc 37% \n*percent daily values are based on a 2000 calorie diet\n(averaged over different types of turkey, roasted, with skin)"
                                }
                            ],
                            "expressiontypes":{
                                "name":"Default"
                            }
                        }
                    ],
                    "assumptions":[
                        {
                            "type":"FormulaVariable",
                            "desc":"meal proportions",
                            "current":"1",
                            "count":2,
                            "values":[
                                {
                                    "name":"Meal:Light",
                                    "desc":"light",
                                    "valid":"true",
                                    "input":"*FP.TurkeyPortions.Type-_Meal%3ALight"
                                },
                                {
                                    "name":"Meal:Heavy",
                                    "desc":"heavy",
                                    "valid":"true",
                                    "input":"*FP.TurkeyPortions.Type-_Meal%3AHeavy"
                                }
                            ]
                        },
                        {
                            "type":"FormulaVariableInclude",
                            "template":"Also include ${desc1}",
                            "count":2,
                            "values":[
                                {
                                    "name":"TurkeyPortions.c",
                                    "desc":"number of children",
                                    "input":"*FVarOpt-_**TurkeyPortions.c--"
                                },
                                {
                                    "name":"TurkeyPortions.t",
                                    "desc":"number of teenagers",
                                    "input":"*FVarOpt-_**TurkeyPortions.t--"
                                }
                            ]
                        }
                    ],
                    "sources":{
                        "url":"https:\/\/www6b3.wolframalpha.com\/sources\/ExpandedFoodDataSourceInformationNotes.html",
                        "text":"Expanded food data"
                    }
                }
            }"#).unwrap();

    let l: RawApiResponse = serde_json::from_str(
                r#"{
                    "queryresult":{
                        "success":true,
                        "error":false,
                        "numpods":11,
                        "datatypes":"Chemical,Element",
                        "timedout":"",
                        "timedoutpods":"",
                        "timing":2.469,
                        "parsetiming":0.193,
                        "parsetimedout":false,
                        "recalculate":"",
                        "id":"MSP3875147733a2gc7aaf02000068cc8ic8eg0348gb",
                        "host":"https:\/\/www6b3.wolframalpha.com",
                        "server":"16",
                        "related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa3876147733a2gc7aaf02000058fh7bcege707af38350669649384844499",
                        "version":"2.6",
                        "inputstring":"InChI=1\/C8H8O3\/c1-11-8-4-6(5-9)2-3-7(8)10\/h2-5,10H,1H3",
                        "pods":[
                            {
                                "title":"Input interpretation",
                                "scanner":"Identity",
                                "id":"Input",
                                "position":100,
                                "error":false,
                                "numsubpods":1,
                                "subpods":[
                                    {
                                        "title":"",
                                        "img":{
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3877147733a2gc7aaf02000030ii45gc524gf1e0?MSPStoreType=image\/gif&s=16",
                                            "alt":"vanillin",
                                            "title":"vanillin",
                                            "width":50,
                                            "height":19,
                                            "type":"Default",
                                            "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                            "colorinvertable":true,
                                            "contenttype":"image\/gif"
                                        },
                                        "plaintext":"vanillin"
                                    }
                                ],
                                "expressiontypes":{
                                    "name":"Default"
                                }
                            },
                            {
                                "title":"Chemical names and formulas",
                                "scanner":"Data",
                                "id":"ChemicalNamesFormulas:ChemicalData",
                                "position":200,
                                "error":false,
                                "numsubpods":1,
                                "primary":true,
                                "subpods":[
                                    {
                                        "title":"",
                                        "microsources":{
                                            "microsource":"ChemicalData"
                                        },
                                        "datasources":{
                                            "datasource":"PubChem"
                                        },
                                        "img":{
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3878147733a2gc7aaf02000033dg0b9ea78dda1d?MSPStoreType=image\/gif&s=16",
                                            "alt":"formula | (HO)C_6H_3(OCH_3)CHO\nHill formula | C_8H_8O_3\nname | vanillin\nIUPAC name | 4-hydroxy-3-methoxybenzaldehyde",
                                            "title":"formula | (HO)C_6H_3(OCH_3)CHO\nHill formula | C_8H_8O_3\nname | vanillin\nIUPAC name | 4-hydroxy-3-methoxybenzaldehyde",
                                            "width":375,
                                            "height":138,
                                            "type":"Grid",
                                            "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                            "colorinvertable":true,
                                            "contenttype":"image\/gif"
                                        },
                                        "plaintext":"formula | (HO)C_6H_3(OCH_3)CHO\nHill formula | C_8H_8O_3\nname | vanillin\nIUPAC name | 4-hydroxy-3-methoxybenzaldehyde"
                                    }
                                ],
                                "expressiontypes":{
                                    "name":"Grid"
                                },
                                "states":[
                                    {
                                        "name":"More",
                                        "input":"ChemicalNamesFormulas:ChemicalData__More"
                                    }
                                ]
                            },
                            {
                                "title":"Structure diagram",
                                "scanner":"Data",
                                "id":"StructureDiagramPod:ChemicalData",
                                "position":300,
                                "error":false,
                                "numsubpods":1,
                                "subpods":[
                                    {
                                        "title":"",
                                        "microsources":{
                                            "microsource":"ChemicalData"
                                        },
                                        "datasources":{
                                            "datasource":"PubChem"
                                        },
                                        "img":{
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3879147733a2gc7aaf0200004513ed2gh2ad40ac?MSPStoreType=image\/gif&s=16",
                                            "alt":"Structure diagram",
                                            "title":"",
                                            "width":183,
                                            "height":130,
                                            "type":"Default",
                                            "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                            "colorinvertable":true,
                                            "contenttype":"image\/gif"
                                        },
                                        "plaintext":""
                                    }
                                ],
                                "expressiontypes":{
                                    "name":"Default"
                                },
                                "states":[
                                    {
                                        "count":3,
                                        "value":"Skeletal structure",
                                        "delimiters":"",
                                        "states":[
                                            {
                                                "name":"Skeletal structure",
                                                "input":"StructureDiagramPod:ChemicalData__Skeletal structure"
                                            },
                                            {
                                                "name":"All atoms",
                                                "input":"StructureDiagramPod:ChemicalData__All atoms"
                                            },
                                            {
                                                "name":"Lewis structure",
                                                "input":"StructureDiagramPod:ChemicalData__Lewis structure"
                                            }
                                        ]
                                    },
                                    {
                                        "name":"Show bond information",
                                        "input":"StructureDiagramPod:ChemicalData__Show bond information"
                                    },
                                    {
                                        "name":"Show graph properties",
                                        "input":"StructureDiagramPod:ChemicalData__Show graph properties"
                                    },
                                    {
                                        "name":"Step-by-step",
                                        "input":"StructureDiagramPod:ChemicalData__Step-by-step",
                                        "stepbystep":true
                                    }
                                ]
                            },
                            {
                                "title":"3D structure",
                                "scanner":"Data",
                                "id":"3DStructure:ChemicalData",
                                "position":400,
                                "error":false,
                                "numsubpods":1,
                                "subpods":[
                                    {
                                        "title":"",
                                        "microsources":{
                                            "microsource":"ChemicalData"
                                        },
                                        "datasources":{
                                            "datasource":"PubChem"
                                        },
                                        "img":{
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3880147733a2gc7aaf020000302d15fg85gf3dba?MSPStoreType=image\/gif&s=16",
                                            "alt":"3D structure",
                                            "title":"",
                                            "width":400,
                                            "height":254,
                                            "type":"Default",
                                            "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                            "colorinvertable":false,
                                            "contenttype":"image\/gif"
                                        },
                                        "plaintext":""
                                    }
                                ],
                                "expressiontypes":{
                                    "name":"Default"
                                },
                                "states":[
                                    {
                                        "name":"Show bonds only",
                                        "input":"3DStructure:ChemicalData__Show bonds only"
                                    },
                                    {
                                        "name":"Show space filling model",
                                        "input":"3DStructure:ChemicalData__Show space filling model"
                                    }
                                ]
                            },
                            {
                                "title":"Basic properties",
                                "scanner":"Data",
                                "id":"Basic:ChemicalData",
                                "position":500,
                                "error":false,
                                "numsubpods":1,
                                "subpods":[
                                    {
                                        "title":"",
                                        "microsources":{
                                            "microsource":"ChemicalData"
                                        },
                                        "datasources":{
                                            "datasource":[
                                                "SOLV-DB",
                                                "PubChem",
                                                "CRCHandbook"
                                            ]
                                        },
                                        "img":{
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3881147733a2gc7aaf0200004ce3g3ggcahfi441?MSPStoreType=image\/gif&s=16",
                                            "alt":"molar mass | 152.15 g\/mol\nphase | solid (at STP)\nmelting point | 82 °C\nboiling point | 170 °C (measured at 2000 Pa)\ndensity | 1.056 g\/cm^3",
                                            "title":"molar mass | 152.15 g\/mol\nphase | solid (at STP)\nmelting point | 82 °C\nboiling point | 170 °C (measured at 2000 Pa)\ndensity | 1.056 g\/cm^3",
                                            "width":312,
                                            "height":169,
                                            "type":"Grid",
                                            "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                            "colorinvertable":true,
                                            "contenttype":"image\/gif"
                                        },
                                        "plaintext":"molar mass | 152.15 g\/mol\nphase | solid (at STP)\nmelting point | 82 °C\nboiling point | 170 °C (measured at 2000 Pa)\ndensity | 1.056 g\/cm^3"
                                    }
                                ],
                                "expressiontypes":{
                                    "name":"Grid"
                                },
                                "states":[
                                    {
                                        "name":"Step-by-step",
                                        "input":"Basic:ChemicalData__Step-by-step",
                                        "stepbystep":true
                                    }
                                ],
                                "infos":{
                                    "units":[
                                        [
                                            {
                                                "short":"g\/cm^3",
                                                "long":"grams per cubic centimeter"
                                            },
                                            {
                                                "short":"g\/mol",
                                                "long":"grams per mole"
                                            },
                                            {
                                                "short":"Pa",
                                                "long":"pascals"
                                            }
                                        ],
                                        {
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3882147733a2gc7aaf0200004ge86717eag4bf82?MSPStoreType=image\/gif&s=16",
                                            "width":"234",
                                            "height":"72"
                                        }
                                    ]
                                }
                            },
                            {
                                "title":"Solid properties (at STP)",
                                "scanner":"Data",
                                "id":"SolidProperties:ChemicalData",
                                "position":600,
                                "error":false,
                                "numsubpods":1,
                                "subpods":[
                                    {
                                        "title":"",
                                        "microsources":{
                                            "microsource":"ChemicalData"
                                        },
                                        "datasources":{
                                            "datasource":"CRCHandbook"
                                        },
                                        "img":{
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3883147733a2gc7aaf0200004f63a3ediegi6bh9?MSPStoreType=image\/gif&s=16",
                                            "alt":"density | 1.056 g\/cm^3\nvapor pressure | 0.009998 mmHg\nrefractive index | 1.555",
                                            "title":"density | 1.056 g\/cm^3\nvapor pressure | 0.009998 mmHg\nrefractive index | 1.555",
                                            "width":257,
                                            "height":103,
                                            "type":"Grid",
                                            "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                            "colorinvertable":true,
                                            "contenttype":"image\/gif"
                                        },
                                        "plaintext":"density | 1.056 g\/cm^3\nvapor pressure | 0.009998 mmHg\nrefractive index | 1.555"
                                    }
                                ],
                                "expressiontypes":{
                                    "name":"Grid"
                                }
                            },
                            {
                                "title":"Thermodynamic properties",
                                "scanner":"Data",
                                "id":"Thermodynamics:ChemicalData",
                                "position":700,
                                "error":false,
                                "numsubpods":1,
                                "subpods":[
                                    {
                                        "title":"",
                                        "microsources":{
                                            "microsource":[
                                                "ChemicalData",
                                                "ElementData"
                                            ]
                                        },
                                        "datasources":{
                                            "datasource":[
                                                "PubChem",
                                                "WebElements"
                                            ]
                                        },
                                        "img":{
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3884147733a2gc7aaf02000053ihi8804ea4e8ca?MSPStoreType=image\/gif&s=16",
                                            "alt":"specific heat of vaporization | 0.357 kJ\/g\nspecific heat of combustion | 12.16 kJ\/g\n(at STP)",
                                            "title":"specific heat of vaporization | 0.357 kJ\/g\nspecific heat of combustion | 12.16 kJ\/g\n(at STP)",
                                            "width":296,
                                            "height":95,
                                            "type":"Grid",
                                            "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                            "colorinvertable":true,
                                            "contenttype":"image\/gif"
                                        },
                                        "plaintext":"specific heat of vaporization | 0.357 kJ\/g\nspecific heat of combustion | 12.16 kJ\/g\n(at STP)"
                                    }
                                ],
                                "expressiontypes":{
                                    "name":"Grid"
                                },
                                "states":[
                                    {
                                        "name":"More",
                                        "input":"Thermodynamics:ChemicalData__More"
                                    }
                                ],
                                "infos":{
                                    "units":[
                                        {
                                            "short":"kJ\/g",
                                            "long":"kilojoules per gram"
                                        },
                                        {
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3885147733a2gc7aaf02000026i0436f6ef9394a?MSPStoreType=image\/gif&s=16",
                                            "width":"172",
                                            "height":"27"
                                        }
                                    ]
                                }
                            },
                            {
                                "title":"Chemical identifiers",
                                "scanner":"Data",
                                "id":"ChemicalIdentifiers:ChemicalData",
                                "position":800,
                                "error":false,
                                "numsubpods":1,
                                "subpods":[
                                    {
                                        "title":"",
                                        "microsources":{
                                            "microsource":"ChemicalData"
                                        },
                                        "datasources":{
                                            "datasource":"PubChem"
                                        },
                                        "img":{
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3886147733a2gc7aaf02000060h507die63id7ed?MSPStoreType=image\/gif&s=16",
                                            "alt":"CAS number | 121-33-5\nBeilstein number | 472792\nPubChem CID number | 1183\nPubChem SID number | 24900708\nSMILES identifier | COC1=C(C=CC(=C1)C=O)O",
                                            "title":"CAS number | 121-33-5\nBeilstein number | 472792\nPubChem CID number | 1183\nPubChem SID number | 24900708\nSMILES identifier | COC1=C(C=CC(=C1)C=O)O",
                                            "width":375,
                                            "height":169,
                                            "type":"Grid",
                                            "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                            "colorinvertable":true,
                                            "contenttype":"image\/gif"
                                        },
                                        "plaintext":"CAS number | 121-33-5\nBeilstein number | 472792\nPubChem CID number | 1183\nPubChem SID number | 24900708\nSMILES identifier | COC1=C(C=CC(=C1)C=O)O"
                                    }
                                ],
                                "expressiontypes":{
                                    "name":"Grid"
                                },
                                "states":[
                                    {
                                        "name":"More",
                                        "input":"ChemicalIdentifiers:ChemicalData__More"
                                    }
                                ]
                            },
                            {
                                "title":"NFPA label",
                                "scanner":"Data",
                                "id":"NFPALabel:ChemicalData",
                                "position":900,
                                "error":false,
                                "numsubpods":1,
                                "subpods":[
                                    {
                                        "title":"",
                                        "microsources":{
                                            "microsource":"ChemicalData"
                                        },
                                        "img":{
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3887147733a2gc7aaf0200005d2agd177d0d85ai?MSPStoreType=image\/gif&s=16",
                                            "alt":"NFPA label",
                                            "title":"",
                                            "width":40,
                                            "height":40,
                                            "type":"Default",
                                            "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                            "colorinvertable":true,
                                            "contenttype":"image\/gif"
                                        },
                                        "plaintext":""
                                    }
                                ],
                                "expressiontypes":{
                                    "name":"Default"
                                },
                                "states":[
                                    {
                                        "name":"Table",
                                        "input":"NFPALabel:ChemicalData__Table"
                                    }
                                ]
                            },
                            {
                                "title":"Safety properties",
                                "scanner":"Data",
                                "id":"SafetyProperties:ChemicalData",
                                "position":1000,
                                "error":false,
                                "numsubpods":1,
                                "subpods":[
                                    {
                                        "title":"",
                                        "microsources":{
                                            "microsource":"ChemicalData"
                                        },
                                        "img":{
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3888147733a2gc7aaf02000068icb0biadfh28f0?MSPStoreType=image\/gif&s=16",
                                            "alt":"flash point | 118 °C\nautoignition point | 400 °C\nlower explosive limit | 1.2% (concentration in air)\nupper explosive limit | 8.8% (concentration in air)",
                                            "title":"flash point | 118 °C\nautoignition point | 400 °C\nlower explosive limit | 1.2% (concentration in air)\nupper explosive limit | 8.8% (concentration in air)",
                                            "width":341,
                                            "height":136,
                                            "type":"Grid",
                                            "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                            "colorinvertable":true,
                                            "contenttype":"image\/gif"
                                        },
                                        "plaintext":"flash point | 118 °C\nautoignition point | 400 °C\nlower explosive limit | 1.2% (concentration in air)\nupper explosive limit | 8.8% (concentration in air)"
                                    }
                                ],
                                "expressiontypes":{
                                    "name":"Grid"
                                }
                            },
                            {
                                "title":"Toxicity properties",
                                "scanner":"Data",
                                "id":"ToxicityProperties:ChemicalData",
                                "position":1100,
                                "error":false,
                                "numsubpods":1,
                                "subpods":[
                                    {
                                        "title":"",
                                        "microsources":{
                                            "microsource":"ChemicalData"
                                        },
                                        "datasources":{
                                            "datasource":"MacmillansChemicalAndPhysicalData"
                                        },
                                        "img":{
                                            "src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP3889147733a2gc7aaf0200004g0db708055134fg?MSPStoreType=image\/gif&s=16",
                                            "alt":"RTECS classes | mutagen | reproductive effector | natural product",
                                            "title":"RTECS classes | mutagen | reproductive effector | natural product",
                                            "width":490,
                                            "height":37,
                                            "type":"Grid",
                                            "themes":"1,2,3,4,5,6,7,8,9,10,11,12",
                                            "colorinvertable":true,
                                            "contenttype":"image\/gif"
                                        },
                                        "plaintext":"RTECS classes | mutagen | reproductive effector | natural product"
                                    }
                                ],
                                "expressiontypes":{
                                    "name":"Grid"
                                }
                            }
                        ],
                        "sources":[
                            {
                                "url":"https:\/\/www6b3.wolframalpha.com\/sources\/ChemicalDataSourceInformationNotes.html",
                                "text":"Chemical data"
                            },
                            {
                                "url":"https:\/\/www6b3.wolframalpha.com\/sources\/ElementDataSourceInformationNotes.html",
                                "text":"Element data"
                            }
                        ]
                    }
                }"#).unwrap();

    let m: RawApiResponse = serde_json::from_str(r#"{
					"queryresult":{
						"success":true,
						"error":false,
						"numpods":6,
						"datatypes":"Agriculture,Country",
						"timedout":"Identity,Data,Percent,Unit,AtmosphericProperties,UnitInformation,Music,Geometry",
						"timedoutpods":"",
						"timing":8.735,
						"parsetiming":0.505,
						"parsetimedout":false,
						"recalculate":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/recalc.jsp?id=MSPa401019a612230cfec83300005gb18ih9e96h41ha1697175660060143921&output=JSON",
						"id":"MSP401119a612230cfec83300001cgeda674bb68928",
						"host":"https:\/\/www6b3.wolframalpha.com",
						"server":"18",
						"related":"https:\/\/www6b3.wolframalpha.com\/api\/v1\/relatedQueries.jsp?id=MSPa401219a612230cfec83300002e41hg4092e8a4b01697175660060143921",
						"version":"2.6",
						"inputstring":"corn production",
						"pods":[
							{
								"title":"Input interpretation",
								"scanner":"Identity",
								"id":"Input",
								"position":100,
								"error":false,
								"numsubpods":1,
								"subpods":[
									{
										"title":"",
										"img":{
											"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP401319a612230cfec833000048cd5dda8hac2g2g?MSPStoreType=image\/gif&s=18",
											"alt":"all countries, dependencies, and territories | production | corn",
											"title":"all countries, dependencies, and territories | production | corn",
											"width":465,
											"height":33,
											"type":"Grid",
											"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
											"colorinvertable":true,
											"contenttype":"image\/gif"
										},
										"plaintext":"all countries, dependencies, and territories | production | corn"
									}
								],
								"expressiontypes":{
									"name":"Grid"
								}
							},
							{
								"title":"Summary",
								"scanner":"Data",
								"id":"Result",
								"position":200,
								"error":false,
								"numsubpods":1,
								"primary":true,
								"subpods":[
									{
										"title":"",
										"microsources":{
											"microsource":"AgricultureData"
										},
										"datasources":{
											"datasource":"FoodAndAgricultureOrganizationOfTheUnitedNationsFoodProductionFAOSTAT"
										},
										"img":{
											"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP401419a612230cfec83300002113dg1e029387ii?MSPStoreType=image\/gif&s=18",
											"alt":"total | 1.149 billion t\/yr\nmean | 6.452 million t\/yr\nhighest | 347 million t\/yr (United States) | (1977, 1980, 2016, and 2019 estimates)\n(based on 178 values; 71 unavailable)",
											"title":"total | 1.149 billion t\/yr\nmean | 6.452 million t\/yr\nhighest | 347 million t\/yr (United States) | (1977, 1980, 2016, and 2019 estimates)\n(based on 178 values; 71 unavailable)",
											"width":531,
											"height":104,
											"type":"Grid",
											"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
											"colorinvertable":true,
											"contenttype":"image\/gif"
										},
										"plaintext":"total | 1.149 billion t\/yr\nmean | 6.452 million t\/yr\nhighest | 347 million t\/yr (United States) | (1977, 1980, 2016, and 2019 estimates)\n(based on 178 values; 71 unavailable)"
									}
								],
								"expressiontypes":{
									"name":"Grid"
								},
								"states":[
									{
										"name":"Show non-metric",
										"input":"Result__Show non-metric"
									},
									{
										"name":"Show details",
										"input":"Result__Show details"
									}
								],
								"infos":{
									"units":[
										{
											"short":"t\/yr",
											"long":"metric tons per year"
										},
										{
											"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP401519a612230cfec833000055bd3b6ii71be064?MSPStoreType=image\/gif&s=18",
											"width":"180",
											"height":"27"
										}
									]
								}
							},
							{
								"title":"Corn production map",
								"scanner":"Data",
								"id":"PropertyMap:AgricultureProduction:AgricultureData",
								"position":300,
								"error":false,
								"numsubpods":1,
								"subpods":[
									{
										"title":"",
										"microsources":{
											"microsource":[
												"AgricultureData",
												"CountryData"
											]
										},
										"datasources":{
											"datasource":[
												"FoodAndAgricultureOrganizationOfTheUnitedNationsFoodProductionFAOSTAT",
												"CIAFactbook"
											]
										},
										"img":{
											"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP401619a612230cfec83300001b15ec54b5dfafaf?MSPStoreType=image\/gif&s=18",
											"alt":"Corn production map",
											"title":"",
											"width":491,
											"height":337,
											"type":"Grid",
											"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
											"colorinvertable":false,
											"contenttype":"image\/gif"
										},
										"plaintext":""
									}
								],
								"expressiontypes":{
									"name":"Default"
								}
							},
							{
								"title":"Distribution plots",
								"scanner":"Data",
								"id":"DistributionPlots",
								"position":400,
								"error":false,
								"numsubpods":1,
								"subpods":[
									{
										"title":"",
										"img":{
											"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP401719a612230cfec833000045ee7gfe5eigf4h3?MSPStoreType=image\/gif&s=18",
											"alt":" \n(corn production in thousands of metric tons per year)",
											"title":" \n(corn production in thousands of metric tons per year)",
											"width":451,
											"height":184,
											"type":"Default",
											"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
											"colorinvertable":true,
											"contenttype":"image\/gif"
										},
										"plaintext":" \n(corn production in thousands of metric tons per year)"
									}
								],
								"expressiontypes":{
									"name":"Default"
								}
							},
							{
								"title":"Corn production rankings",
								"scanner":"Data",
								"id":"PropertyRanking:AgricultureData",
								"position":500,
								"error":false,
								"numsubpods":1,
								"subpods":[
									{
										"title":"",
										"microsources":{
											"microsource":"AgricultureData"
										},
										"datasources":{
											"datasource":"FoodAndAgricultureOrganizationOfTheUnitedNationsFoodProductionFAOSTAT"
										},
										"img":{
											"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP401819a612230cfec833000016cb77ibggf5h6hd?MSPStoreType=image\/gif&s=18",
											"alt":"1 | United States | 347 million t\/yr\n2 | China | 260.8 million t\/yr\n3 | Brazil | 101.1 million t\/yr\n4 | Argentina | 56.86 million t\/yr\n5 | Ukraine | 35.88 million t\/yr\n⋮ | | \n168 | Antigua and Barbuda | 80 t\/yr\n169 | Guam | 50 t\/yr\n170 | Barbados | 42 t\/yr\n171 | Puerto Rico | 38 t\/yr\n172 | Djibouti | 21 t\/yr\n(1977, 1980, 2016, and 2019 estimates)\n(based on 172 values; 77 unavailable)",
											"title":"1 | United States | 347 million t\/yr\n2 | China | 260.8 million t\/yr\n3 | Brazil | 101.1 million t\/yr\n4 | Argentina | 56.86 million t\/yr\n5 | Ukraine | 35.88 million t\/yr\n⋮ | | \n168 | Antigua and Barbuda | 80 t\/yr\n169 | Guam | 50 t\/yr\n170 | Barbados | 42 t\/yr\n171 | Puerto Rico | 38 t\/yr\n172 | Djibouti | 21 t\/yr\n(1977, 1980, 2016, and 2019 estimates)\n(based on 172 values; 77 unavailable)",
											"width":346,
											"height":406,
											"type":"Grid",
											"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
											"colorinvertable":true,
											"contenttype":"image\/gif"
										},
										"plaintext":"1 | United States | 347 million t\/yr\n2 | China | 260.8 million t\/yr\n3 | Brazil | 101.1 million t\/yr\n4 | Argentina | 56.86 million t\/yr\n5 | Ukraine | 35.88 million t\/yr\n⋮ | | \n168 | Antigua and Barbuda | 80 t\/yr\n169 | Guam | 50 t\/yr\n170 | Barbados | 42 t\/yr\n171 | Puerto Rico | 38 t\/yr\n172 | Djibouti | 21 t\/yr\n(1977, 1980, 2016, and 2019 estimates)\n(based on 172 values; 77 unavailable)"
									}
								],
								"expressiontypes":{
									"name":"Grid"
								},
								"states":[
									{
										"name":"More",
										"input":"PropertyRanking:AgricultureData__More"
									},
									{
										"name":"Reverse",
										"input":"PropertyRanking:AgricultureData__Reverse"
									},
									{
										"name":"Show non-metric",
										"input":"PropertyRanking:AgricultureData__Show non-metric"
									}
								],
								"infos":{
									"units":[
										{
											"short":"t\/yr",
											"long":"metric tons per year"
										},
										{
											"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP401919a612230cfec83300002b9fb198c06g010c?MSPStoreType=image\/gif&s=18",
											"width":"180",
											"height":"27"
										}
									]
								}
							},
							{
								"title":"Food production",
								"scanner":"Data",
								"id":"FoodProduction:AgricultureData",
								"position":600,
								"error":false,
								"numsubpods":1,
								"subpods":[
									{
										"title":"Common foods",
										"microsources":{
											"microsource":"AgricultureData"
										},
										"datasources":{
											"datasource":"FoodAndAgricultureOrganizationOfTheUnitedNationsFoodProductionFAOSTAT"
										},
										"img":{
											"src":"https:\/\/www6b3.wolframalpha.com\/Calculate\/MSP\/MSP402019a612230cfec83300003a60g5hcd73a16ig?MSPStoreType=image\/gif&s=18",
											"alt":"beans | total | 26.87 million t\/yr (metric tons per year)\n | mean | 205133 t\/yr (metric tons per year)\n | highest | 5.19 million t\/yr (metric tons per year) (Myanmar)\n | lowest | 1 t\/yr (metric ton per year) (Czech Republic)\ncoffee | total | 10.04 million t\/yr (metric tons per year)\n | mean | 118104 t\/yr (metric tons per year)\n | highest | 3.009 million t\/yr (metric tons per year) (Brazil)\neggs | total | 88.36 million t\/yr (metric tons per year)\n | mean | 424805 t\/yr (metric tons per year)\n | highest | 33.09 million t\/yr (metric tons per year) (China)\n | lowest | 7 t\/yr (metric tons per year) (Cayman Islands)\nmilk | total | 880.9 million t\/yr (metric tons per year)\n | mean | 4.427 million t\/yr (metric tons per year)\n | highest | 187.6 million t\/yr (metric tons per year) (India)\n | lowest | 30 t\/yr (metric tons per year) (Wallis and Futuna Islands)\nwheat | total | 765.8 million t\/yr (metric tons per year)\n | mean | 6.126 million t\/yr (metric tons per year)\n | highest | 133.6 million t\/yr (metric tons per year) (China)\n(1964 to 2019 estimates)",
											"title":"beans | total | 26.87 million t\/yr (metric tons per year)\n | mean | 205133 t\/yr (metric tons per year)\n | highest | 5.19 million t\/yr (metric tons per year) (Myanmar)\n | lowest | 1 t\/yr (metric ton per year) (Czech Republic)\ncoffee | total | 10.04 million t\/yr (metric tons per year)\n | mean | 118104 t\/yr (metric tons per year)\n | highest | 3.009 million t\/yr (metric tons per year) (Brazil)\neggs | total | 88.36 million t\/yr (metric tons per year)\n | mean | 424805 t\/yr (metric tons per year)\n | highest | 33.09 million t\/yr (metric tons per year) (China)\n | lowest | 7 t\/yr (metric tons per year) (Cayman Islands)\nmilk | total | 880.9 million t\/yr (metric tons per year)\n | mean | 4.427 million t\/yr (metric tons per year)\n | highest | 187.6 million t\/yr (metric tons per year) (India)\n | lowest | 30 t\/yr (metric tons per year) (Wallis and Futuna Islands)\nwheat | total | 765.8 million t\/yr (metric tons per year)\n | mean | 6.126 million t\/yr (metric tons per year)\n | highest | 133.6 million t\/yr (metric tons per year) (China)\n(1964 to 2019 estimates)",
											"width":535,
											"height":571,
											"type":"Grid",
											"themes":"1,2,3,4,5,6,7,8,9,10,11,12",
											"colorinvertable":true,
											"contenttype":"image\/gif"
										},
										"plaintext":"beans | total | 26.87 million t\/yr (metric tons per year)\n | mean | 205133 t\/yr (metric tons per year)\n | highest | 5.19 million t\/yr (metric tons per year) (Myanmar)\n | lowest | 1 t\/yr (metric ton per year) (Czech Republic)\ncoffee | total | 10.04 million t\/yr (metric tons per year)\n | mean | 118104 t\/yr (metric tons per year)\n | highest | 3.009 million t\/yr (metric tons per year) (Brazil)\neggs | total | 88.36 million t\/yr (metric tons per year)\n | mean | 424805 t\/yr (metric tons per year)\n | highest | 33.09 million t\/yr (metric tons per year) (China)\n | lowest | 7 t\/yr (metric tons per year) (Cayman Islands)\nmilk | total | 880.9 million t\/yr (metric tons per year)\n | mean | 4.427 million t\/yr (metric tons per year)\n | highest | 187.6 million t\/yr (metric tons per year) (India)\n | lowest | 30 t\/yr (metric tons per year) (Wallis and Futuna Islands)\nwheat | total | 765.8 million t\/yr (metric tons per year)\n | mean | 6.126 million t\/yr (metric tons per year)\n | highest | 133.6 million t\/yr (metric tons per year) (China)\n(1964 to 2019 estimates)"
									}
								],
								"expressiontypes":{
									"name":"Grid"
								},
								"states":[
									{
										"name":"More",
										"input":"FoodProduction:AgricultureData__More"
									},
									{
										"name":"Show non-metric",
										"input":"FoodProduction:AgricultureData__Show non-metric"
									}
								]
							}
						],
						"sources":[
							{
								"url":"https:\/\/www6b3.wolframalpha.com\/sources\/AgricultureDataSourceInformationNotes.html",
								"text":"Agriculture data"
							},
							{
								"url":"https:\/\/www6b3.wolframalpha.com\/sources\/CountryDataSourceInformationNotes.html",
								"text":"Country data"
							}
						]
					}
				}"#).unwrap();
}

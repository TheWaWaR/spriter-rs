use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename = "spriter_data")]
pub struct Spriter {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterFolder {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterFile {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterEntity {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterObjectInfo {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterAnimation {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterMainlineKey {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterRef {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterObjectRef {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterTimeline {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterTimelineKey {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterSpatial {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterObject {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterCharacterMap {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterMapInstruction {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterMeta {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterVarDef {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterVarline {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterVarlineKey {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterVarValue {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterEventline {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterTagline {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterTaglineKey {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterTag {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterSoundline {}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterSoundlineKey {
    #[serde(flatten)]
    pub key: SpriterKey,
    #[serde(rename = "object")]
    pub sound_object: SpriterSound,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterSound {
    #[serde(flatten)]
    pub elem: SpriterElement,
    #[serde(rename = "folder")]
    pub folder_id: i64,
    #[serde(rename = "file")]
    pub file_id: i64,
    pub trigger: bool,
    pub panning: f64,
    pub volume: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterElement {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriterKey {
    #[serde(flatten)]
    pub elem: SpriterElement,
    pub time: f64,
    pub curve_type: SpriterCurveType,
    pub c1: f64,
    pub c2: f64,
    pub c3: f64,
    pub c4: f64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SpriterObjectType {
    Sprite,
    Bone,
    Box,
    Point,
    Sound,
    Entity,
    Variable,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SpriterCurveType {
    Linear,
    Instant,
    Quadratic,
    Cubic,
    Quartic,
    Quintic,
    Bezier,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SpriterFileType {
    Image,
    Sound,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SpriterVarType {
    String,
    Int,
    Float,
}

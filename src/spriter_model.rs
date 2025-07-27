use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "spriter_data")]
pub struct Spriter {
    #[serde(rename = "folder", default)]
    pub folders: Vec<SpriterFolder>,

    #[serde(rename = "entity", default)]
    pub entities: Vec<SpriterEntity>,

    #[serde(rename = "tag_list")]
    pub tags: Option<TagList>,

    #[serde(rename = "atlas")]
    pub atlases: Option<AtlasList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagList {
    #[serde(rename = "i", default)]
    pub items: Vec<SpriterElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlasList {
    #[serde(rename = "i", default)]
    pub items: Vec<SpriterElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterFolder {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "file", default)]
    pub files: Vec<SpriterFile>,

    #[serde(rename = "@atlas", default = "default_atlas_id")]
    pub atlas_id: i32,
}

fn default_atlas_id() -> i32 {
    -1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterFile {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@type", default)]
    pub file_type: SpriterFileType,

    #[serde(rename = "@pivot_x", default)]
    pub pivot_x: f32,

    #[serde(rename = "@pivot_y", default = "default_pivot_y")]
    pub pivot_y: f32,

    #[serde(rename = "@width", default)]
    pub width: i32,

    #[serde(rename = "@height", default)]
    pub height: i32,
}

fn default_pivot_y() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterEntity {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(skip)]
    pub spriter: Option<*const Spriter>,

    #[serde(rename = "obj_info", default)]
    pub object_infos: Vec<SpriterObjectInfo>,

    #[serde(rename = "character_map", default)]
    pub character_maps: Vec<SpriterCharacterMap>,

    #[serde(rename = "animation", default)]
    pub animations: Vec<SpriterAnimation>,

    #[serde(rename = "var_defs")]
    pub variables: Option<VarDefsList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarDefsList {
    #[serde(rename = "i", default)]
    pub items: Vec<SpriterVarDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterObjectInfo {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@type", default)]
    pub object_type: SpriterObjectType,

    #[serde(rename = "@w", default)]
    pub width: f32,

    #[serde(rename = "@h", default)]
    pub height: f32,

    #[serde(rename = "@pivot_x", default)]
    pub pivot_x: f32,

    #[serde(rename = "@pivot_y", default)]
    pub pivot_y: f32,

    #[serde(rename = "var_defs")]
    pub variables: Option<VarDefsList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterAnimation {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(skip)]
    pub entity: Option<*const SpriterEntity>,

    #[serde(rename = "@length", default)]
    pub length: f32,

    #[serde(rename = "@looping", default = "default_looping")]
    pub looping: bool,

    #[serde(rename = "mainline")]
    pub mainline_keys: Option<MainlineKeysList>,

    #[serde(rename = "timeline", default)]
    pub timelines: Vec<SpriterTimeline>,

    #[serde(rename = "eventline", default)]
    pub eventlines: Vec<SpriterEventline>,

    #[serde(rename = "soundline", default)]
    pub soundlines: Vec<SpriterSoundline>,

    #[serde(rename = "meta")]
    pub meta: Option<SpriterMeta>,
}

fn default_looping() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainlineKeysList {
    #[serde(rename = "key", default)]
    pub keys: Vec<SpriterMainlineKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterMainlineKey {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@time", default)]
    pub time: f32,

    #[serde(rename = "@curve_type", default)]
    pub curve_type: SpriterCurveType,

    #[serde(rename = "@c1", default)]
    pub c1: f32,

    #[serde(rename = "@c2", default)]
    pub c2: f32,

    #[serde(rename = "@c3", default)]
    pub c3: f32,

    #[serde(rename = "@c4", default)]
    pub c4: f32,

    #[serde(rename = "bone_ref", default)]
    pub bone_refs: Vec<SpriterRef>,

    #[serde(rename = "object_ref", default)]
    pub object_refs: Vec<SpriterObjectRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterRef {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@parent", default = "default_parent_id")]
    pub parent_id: i32,

    #[serde(rename = "@timeline", default)]
    pub timeline_id: i32,

    #[serde(rename = "@key", default)]
    pub key_id: i32,
}

fn default_parent_id() -> i32 {
    -1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterObjectRef {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@parent", default = "default_parent_id")]
    pub parent_id: i32,

    #[serde(rename = "@timeline", default)]
    pub timeline_id: i32,

    #[serde(rename = "@key", default)]
    pub key_id: i32,

    #[serde(rename = "@z_index", default)]
    pub z_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterTimeline {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@object_type", default)]
    pub object_type: SpriterObjectType,

    #[serde(rename = "@obj", default)]
    pub object_id: i32,

    #[serde(rename = "key", default)]
    pub keys: Vec<SpriterTimelineKey>,

    #[serde(rename = "meta")]
    pub meta: Option<SpriterMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterTimelineKey {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@time", default)]
    pub time: f32,

    #[serde(rename = "@curve_type", default)]
    pub curve_type: SpriterCurveType,

    #[serde(rename = "@c1", default)]
    pub c1: f32,

    #[serde(rename = "@c2", default)]
    pub c2: f32,

    #[serde(rename = "@c3", default)]
    pub c3: f32,

    #[serde(rename = "@c4", default)]
    pub c4: f32,

    #[serde(rename = "@spin", default = "default_spin")]
    pub spin: i32,

    #[serde(rename = "bone")]
    pub bone_info: Option<SpriterSpatial>,

    #[serde(rename = "object")]
    pub object_info: Option<SpriterObject>,
}

fn default_spin() -> i32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterSpatial {
    #[serde(rename = "@x", default)]
    pub x: f32,

    #[serde(rename = "@y", default)]
    pub y: f32,

    #[serde(rename = "@angle", default)]
    pub angle: f32,

    #[serde(rename = "@scale_x", default = "default_scale")]
    pub scale_x: f32,

    #[serde(rename = "@scale_y", default = "default_scale")]
    pub scale_y: f32,

    #[serde(rename = "@a", default = "default_alpha")]
    pub alpha: f32,
}

fn default_scale() -> f32 {
    1.0
}

fn default_alpha() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterObject {
    #[serde(rename = "@x", default)]
    pub x: f32,

    #[serde(rename = "@y", default)]
    pub y: f32,

    #[serde(rename = "@angle", default)]
    pub angle: f32,

    #[serde(rename = "@scale_x", default = "default_scale")]
    pub scale_x: f32,

    #[serde(rename = "@scale_y", default = "default_scale")]
    pub scale_y: f32,

    #[serde(rename = "@a", default = "default_alpha")]
    pub alpha: f32,

    #[serde(rename = "@animation", default)]
    pub animation_id: i32,

    #[serde(rename = "@entity", default)]
    pub entity_id: i32,

    #[serde(rename = "@folder", default)]
    pub folder_id: i32,

    #[serde(rename = "@file", default)]
    pub file_id: i32,

    #[serde(rename = "@pivot_x", default = "default_nan")]
    pub pivot_x: f32,

    #[serde(rename = "@pivot_y", default = "default_nan")]
    pub pivot_y: f32,

    #[serde(rename = "@t", default)]
    pub t: f32,
}

fn default_nan() -> f32 {
    f32::NAN
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterCharacterMap {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "map", default)]
    pub maps: Vec<SpriterMapInstruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterMapInstruction {
    #[serde(rename = "@folder", default)]
    pub folder_id: i32,

    #[serde(rename = "@file", default)]
    pub file_id: i32,

    #[serde(rename = "@target_folder", default = "default_target_folder")]
    pub target_folder_id: i32,

    #[serde(rename = "@target_file", default = "default_target_file")]
    pub target_file_id: i32,
}

fn default_target_folder() -> i32 {
    -1
}

fn default_target_file() -> i32 {
    -1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterMeta {
    #[serde(rename = "varline", default)]
    pub varlines: Vec<SpriterVarline>,

    #[serde(rename = "tagline")]
    pub tagline: Option<SpriterTagline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterVarDef {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@type", default)]
    pub var_type: SpriterVarType,

    #[serde(rename = "@default")]
    pub default_value: Option<String>,

    #[serde(skip)]
    pub variable_value: Option<SpriterVarValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterVarline {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@def", default)]
    pub def: i32,

    #[serde(rename = "key", default)]
    pub keys: Vec<SpriterVarlineKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterVarlineKey {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@time", default)]
    pub time: f32,

    #[serde(rename = "@curve_type", default)]
    pub curve_type: SpriterCurveType,

    #[serde(rename = "@c1", default)]
    pub c1: f32,

    #[serde(rename = "@c2", default)]
    pub c2: f32,

    #[serde(rename = "@c3", default)]
    pub c3: f32,

    #[serde(rename = "@c4", default)]
    pub c4: f32,

    #[serde(rename = "@val")]
    pub value: Option<String>,

    #[serde(skip)]
    pub variable_value: Option<SpriterVarValue>,
}

#[derive(Debug, Clone)]
pub struct SpriterVarValue {
    pub var_type: SpriterVarType,
    pub string_value: Option<String>,
    pub float_value: f32,
    pub int_value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterEventline {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "key", default)]
    pub keys: Vec<SpriterKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterTagline {
    #[serde(rename = "key", default)]
    pub keys: Vec<SpriterTaglineKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterTaglineKey {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@time", default)]
    pub time: f32,

    #[serde(rename = "@curve_type", default)]
    pub curve_type: SpriterCurveType,

    #[serde(rename = "@c1", default)]
    pub c1: f32,

    #[serde(rename = "@c2", default)]
    pub c2: f32,

    #[serde(rename = "@c3", default)]
    pub c3: f32,

    #[serde(rename = "@c4", default)]
    pub c4: f32,

    #[serde(rename = "tag", default)]
    pub tags: Vec<SpriterTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterTag {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@t", default)]
    pub tag_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterSoundline {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "key", default)]
    pub keys: Vec<SpriterSoundlineKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterSoundlineKey {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@time", default)]
    pub time: f32,

    #[serde(rename = "@curve_type", default)]
    pub curve_type: SpriterCurveType,

    #[serde(rename = "@c1", default)]
    pub c1: f32,

    #[serde(rename = "@c2", default)]
    pub c2: f32,

    #[serde(rename = "@c3", default)]
    pub c3: f32,

    #[serde(rename = "@c4", default)]
    pub c4: f32,

    #[serde(rename = "object")]
    pub sound_object: Option<SpriterSound>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterSound {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@folder", default)]
    pub folder_id: i32,

    #[serde(rename = "@file", default)]
    pub file_id: i32,

    #[serde(rename = "@trigger", default = "default_trigger")]
    pub trigger: bool,

    #[serde(rename = "@panning", default)]
    pub panning: f32,

    #[serde(rename = "@volume", default = "default_volume")]
    pub volume: f32,
}

fn default_trigger() -> bool {
    true
}

fn default_volume() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterElement {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriterKey {
    #[serde(rename = "@id")]
    pub id: i32,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@time", default)]
    pub time: f32,

    #[serde(rename = "@curve_type", default)]
    pub curve_type: SpriterCurveType,

    #[serde(rename = "@c1", default)]
    pub c1: f32,

    #[serde(rename = "@c2", default)]
    pub c2: f32,

    #[serde(rename = "@c3", default)]
    pub c3: f32,

    #[serde(rename = "@c4", default)]
    pub c4: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SpriterObjectType {
    #[serde(rename = "sprite")]
    Sprite,
    #[serde(rename = "bone")]
    Bone,
    #[serde(rename = "box")]
    Box,
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "sound")]
    Sound,
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "variable")]
    Variable,
}

impl Default for SpriterObjectType {
    fn default() -> Self {
        SpriterObjectType::Sprite
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SpriterCurveType {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "instant")]
    Instant,
    #[serde(rename = "quadratic")]
    Quadratic,
    #[serde(rename = "cubic")]
    Cubic,
    #[serde(rename = "quartic")]
    Quartic,
    #[serde(rename = "quintic")]
    Quintic,
    #[serde(rename = "bezier")]
    Bezier,
}

impl Default for SpriterCurveType {
    fn default() -> Self {
        SpriterCurveType::Linear
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SpriterFileType {
    Image,
    #[serde(rename = "sound")]
    Sound,
}

impl Default for SpriterFileType {
    fn default() -> Self {
        SpriterFileType::Image
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SpriterVarType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "float")]
    Float,
}

impl Default for SpriterVarType {
    fn default() -> Self {
        SpriterVarType::String
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_player_scml() {
        let xml_content =
            fs::read_to_string("src/player.scml").expect("Failed to read player.scml file");

        {
            let jd = &mut quick_xml::de::Deserializer::from_str(&xml_content);
            let result: Result<Spriter, _> = serde_path_to_error::deserialize(jd);
            if let Err(err) = &result {
                let path = err.path().to_string();
                dbg!(path);
            }
        }
        let spriter: Spriter =
            quick_xml::de::from_str(&xml_content).expect("Failed to parse SCML file");

        // Verify basic structure
        assert!(!spriter.folders.is_empty(), "Should have folders");
        assert!(!spriter.entities.is_empty(), "Should have entities");

        // Check first folder
        let first_folder = &spriter.folders[0];
        dbg!(first_folder);
        assert_eq!(first_folder.id, 0);
        assert_eq!(first_folder.name, "torso");
        assert!(
            !first_folder.files.is_empty(),
            "First folder should have files"
        );

        // Check first file in first folder
        let first_file = &first_folder.files[0];
        assert_eq!(first_file.id, 0);
        assert_eq!(first_file.name, "torso/p_torso_idle.png");
        assert_eq!(first_file.width, 88);
        assert_eq!(first_file.height, 88);

        // Check entities exist and have animations
        let first_entity = &spriter.entities[0];
        assert!(
            !first_entity.animations.is_empty(),
            "Entity should have animations"
        );

        println!(
            "Successfully parsed SCML with {} folders and {} entities",
            spriter.folders.len(),
            spriter.entities.len()
        );
    }
}

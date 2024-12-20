#![allow(non_snake_case)]
use babelfont::{DesignLocation, UserLocation};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Serializer;
use std::collections::HashMap;

use wasm_bindgen::{prelude::*, JsValue};

fn serialize_json_compatible<T>(obj: &T) -> Result<JsValue, serde_wasm_bindgen::Error>
where
    T: Serialize,
{
    obj.serialize(&Serializer::json_compatible())
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct FontraAxis {
    name: String,
    label: String,
    tag: String,
    minValue: f32,
    maxValue: f32,
    defaultValue: f32,
    hidden: bool,
}
#[derive(Serialize, Deserialize, Default)]
struct FontraAxes {
    axes: Vec<FontraAxis>,
    mappings: Vec<String>, // Should be a cross-axis mapping
    elidedFallBackname: String,
}

#[derive(Serialize, Deserialize, Default)]
struct FontraGuideline {
    name: Option<String>,
    x: f32,
    y: f32,
    angle: f32,
    locked: bool,
}

#[derive(Serialize, Deserialize, Default)]
struct FontraSource {
    name: String,
    isSparse: String,
    location: HashMap<String, f32>,
    italicAngle: f32,
    guidelines: Vec<FontraGuideline>,
    customData: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default)]
struct FontraBackendInfo {
    features: Vec<String>,
    projectManagerFeatures: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
struct GlyphAxis {}

#[derive(Serialize, Deserialize, Default)]
struct GlyphSource {
    name: String,
    layerName: String,
    location: HashMap<String, f32>,
}

#[derive(Serialize, Deserialize, Default)]
struct Layer {
    glyph: StaticGlyph,
}
#[derive(Serialize, Deserialize, Default)]
struct StaticGlyph {
    path: PackedPath,
    components: Vec<Component>,
    xAdvance: f32,
    yAdvance: f32,
    anchors: Vec<Anchor>,
    guides: Vec<FontraGuideline>,
}
#[derive(Serialize, Deserialize, Default)]
struct Component {
    name: String,
    transformation: DecomposedTransform,
    location: HashMap<String, f32>,
}

#[derive(Serialize, Deserialize, Default)]
struct Anchor {
    name: String,
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Default)]
struct DecomposedTransform {
    translateX: f32,
    translateY: f32,
    rotation: f32,
    scaleX: f32,
    scaleY: f32,
    skewX: f32,
    skewY: f32,
    tCenterX: f32,
    tCenterY: f32,
}
#[derive(Serialize, Deserialize, Default)]
struct ContourInfo {
    endPoint: usize,
    isClosed: bool,
}

#[derive(Serialize, Deserialize, Default)]
struct PackedPath {
    coordinates: Vec<f32>,
    pointTypes: Vec<i32>,
    contourInfo: Vec<ContourInfo>,
}
#[derive(Serialize, Deserialize, Default)]
struct FontraGlyph {
    name: String,
    axes: Vec<GlyphAxis>,
    sources: Vec<GlyphSource>,
    layers: HashMap<String, Layer>,
}

#[derive(Serialize, Deserialize, Default)]
struct FontraFontInfo {
    familyName: Option<String>,
    versionMajor: Option<u16>,
    versionMinor: Option<u16>,
    copyright: Option<String>,
    trademark: Option<String>,
    description: Option<String>,
    sampleText: Option<String>,
    designer: Option<String>,
    designerURL: Option<String>,
    manufacturer: Option<String>,
    manufacturerURL: Option<String>,
    licenseDescription: Option<String>,
    licenseInfoURL: Option<String>,
    vendorID: Option<String>,
    customData: HashMap<String, String>,
}

#[wasm_bindgen]
pub struct Font(babelfont::Font);

#[wasm_bindgen]
impl Font {
    #[wasm_bindgen(constructor)]
    pub fn new(font_a: &str) -> Result<Font, JsValue> {
        let font = babelfont::convertors::glyphs3::load_str(font_a, "".into())
            .map_err(|e| JsValue::from_str(&format!("Error loading font: {:?}", e)))?;
        Ok(Font(font))
    }

    pub fn getGlyphMap(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let map: HashMap<&String, Vec<u32>> = self
            .0
            .glyphs
            .iter()
            .map(|g| (&g.name, g.codepoints.clone()))
            .collect();
        serialize_json_compatible(&map)
    }

    pub fn getAxes(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let mut axes = FontraAxes::default();
        for axis in self.0.axes.iter() {
            let name = axis
                .name
                .get_default()
                .map(|s| s.as_str())
                .unwrap_or("Unknown axis")
                .to_string();
            axes.axes.push(FontraAxis {
                name: axis.tag.to_string(), // XXX: This should be the name, but for expediency
                label: "".to_string(),
                tag: axis.tag.to_string(),
                minValue: axis.min.map(|x| x.to_f32()).unwrap_or(0.0),
                maxValue: axis.max.map(|x| x.to_f32()).unwrap_or(0.0),
                defaultValue: axis.default.map(|x| x.to_f32()).unwrap_or(0.0),
                hidden: axis.hidden,
            });
        }
        serialize_json_compatible(&axes)
    }

    pub fn getBackgroundImage(
        &self,
        _identifier: String,
    ) -> Result<JsValue, serde_wasm_bindgen::Error> {
        Ok(JsValue::NULL)
    }
    pub fn putBackgroundImage(
        &self,
        _identifier: String,
        _image: JsValue,
    ) -> Result<JsValue, serde_wasm_bindgen::Error> {
        Ok(JsValue::NULL)
    }

    pub fn getGlyph(&self, name: String) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let glyph = self
            .0
            .glyphs
            .iter()
            .find(|g| g.name == name)
            .ok_or_else(|| JsValue::from_str("Glyph not found"))?;
        let fontraglyph: FontraGlyph = convert_glyph(glyph, &self.0);
        serialize_json_compatible(&fontraglyph)
    }

    pub fn getSources(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let sources: Vec<FontraSource> = self.0.masters.iter().map(Into::into).collect();
        serialize_json_compatible(&sources)
    }

    pub fn getUnitsPerEm(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serialize_json_compatible(&self.0.upm)
    }

    pub fn isReadOnly(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serialize_json_compatible(&false)
    }

    pub fn getBackendInfo(&self) -> JsValue {
        serialize_json_compatible(&FontraBackendInfo::default()).unwrap()
    }

    pub fn getCustomData(&self) -> JsValue {
        let nothing: HashMap<String, String> = HashMap::new();
        serialize_json_compatible(&nothing).unwrap()
    }

    pub fn subscribeChanges(&self, _path: String, _liveChanges: bool) -> JsValue {
        JsValue::NULL
    }

    pub fn unsubscribeChanges(&self, _path: String, _wantLiveChanges: bool) -> JsValue {
        JsValue::NULL
    }

    pub fn editFinal(
        &self,
        _finalChange: JsValue,
        _rollbackChange: JsValue,
        _editLabel: String,
        _broadcast: bool,
    ) -> JsValue {
        JsValue::NULL
    }

    pub fn editIncremental(&self, _change: JsValue) -> JsValue {
        JsValue::NULL
    }

    pub fn exportAs(&self, _options: JsValue) -> JsValue {
        JsValue::NULL
    }

    pub fn findGlyphsThatUseGlyph(&self, _glyphname: String) -> Vec<String> {
        vec![]
    }
    pub fn on(&self, _event: String, _callback: JsValue) -> JsValue {
        JsValue::NULL
    }

    pub fn getFontInfo(&self) -> JsValue {
        let fontinfo: FontraFontInfo = (&self.0).into();
        serialize_json_compatible(&fontinfo).unwrap()
    }
}

fn convert_glyph(val: &babelfont::Glyph, font: &babelfont::Font) -> FontraGlyph {
    let mut glyph = FontraGlyph {
        name: val.name.clone(),
        axes: vec![],
        sources: vec![],
        layers: HashMap::new(),
    };
    let master_locations: HashMap<String, &DesignLocation> = font
        .masters
        .iter()
        .map(|m| (m.id.clone(), &m.location))
        .collect::<HashMap<String, _>>();
    for layer in val.layers.iter() {
        let layer_id = layer.id.clone().unwrap_or("Unknown layer".to_string());
        glyph.layers.insert(layer_id.clone(), layer.into());
        glyph.sources.push(GlyphSource {
            name: layer_id.clone(),
            layerName: layer_id.clone(),
            location: master_locations
                .get(&layer_id.clone())
                .map(|loc| {
                    loc.iter()
                        .map(|(k, v)| (k.to_string(), v.to_f32()))
                        .collect::<HashMap<String, f32>>()
                })
                .unwrap_or_default(),
        })
    }
    glyph
}

impl From<&babelfont::Layer> for Layer {
    fn from(val: &babelfont::Layer) -> Self {
        let mut path = PackedPath::default();
        for p in val.paths() {
            path.push_path(p);
        }

        Layer {
            glyph: StaticGlyph {
                path,
                components: val.components().map(|c| c.into()).collect(),
                xAdvance: val.width,
                yAdvance: 0.0,
                anchors: val.anchors.iter().map(|a| a.into()).collect(),
                guides: vec![],
            },
        }
    }
}

impl From<&babelfont::Component> for Component {
    fn from(val: &babelfont::Component) -> Self {
        let coeffs = val.transform.as_coeffs();
        Component {
            name: val.reference.clone(),
            transformation: DecomposedTransform {
                translateX: coeffs[4] as f32,
                translateY: coeffs[5] as f32,
                rotation: 0.0,
                scaleX: coeffs[0] as f32,
                scaleY: coeffs[3] as f32,
                skewX: 0.0,
                skewY: 0.0,
                tCenterX: 0.0,
                tCenterY: 0.0,
            },
            location: HashMap::new(),
        }
    }
}

impl From<&babelfont::Anchor> for Anchor {
    fn from(val: &babelfont::Anchor) -> Self {
        Anchor {
            name: val.name.clone(),
            x: val.x,
            y: val.y,
        }
    }
}

impl PackedPath {
    fn push_path(&mut self, babelfont: &babelfont::Path) {
        for node in babelfont.nodes.iter() {
            self.coordinates.push(node.x);
            self.coordinates.push(node.y);
            if node.nodetype != babelfont::NodeType::OffCurve {
                self.pointTypes.push(0);
            } else {
                self.pointTypes.push(1);
            }
        }
        self.contourInfo.push(ContourInfo {
            endPoint: self.coordinates.len() / 2 - 1,
            isClosed: babelfont.closed,
        })
    }
}

impl From<&babelfont::Font> for FontraFontInfo {
    fn from(val: &babelfont::Font) -> Self {
        FontraFontInfo {
            familyName: val.names.family_name.get_default().cloned(),
            versionMajor: Some(val.version.0),
            versionMinor: Some(val.version.1),
            copyright: val.names.copyright.get_default().cloned(),
            trademark: val.names.trademark.get_default().cloned(),
            description: val.names.description.get_default().cloned(),
            sampleText: val.names.sample_text.get_default().cloned(),
            designer: val.names.designer.get_default().cloned(),
            designerURL: val.names.designer_url.get_default().cloned(),
            manufacturer: val.names.manufacturer.get_default().cloned(),
            manufacturerURL: val.names.manufacturer_url.get_default().cloned(),
            licenseDescription: val.names.license.get_default().cloned(),
            licenseInfoURL: val.names.license_url.get_default().cloned(),
            vendorID: None,
            customData: HashMap::new(),
        }
    }
}

impl From<&babelfont::Master> for FontraSource {
    fn from(val: &babelfont::Master) -> Self {
        FontraSource {
            name: val.id.clone(),
            // name: val
            //     .name
            //     .get_default()
            //     .map(|x| x.as_str())
            //     .unwrap_or("Unnamed master")
            //     .to_string(),
            isSparse: "False".to_string(),
            // Location really ought to use axis *name*
            location: val
                .location
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_f32()))
                .collect(),
            italicAngle: 0.0,
            guidelines: val
                .guides
                .iter()
                .map(|g| g.into())
                .collect::<Vec<FontraGuideline>>(),
            customData: HashMap::new(),
        }
    }
}

impl From<&babelfont::Guide> for FontraGuideline {
    fn from(val: &babelfont::Guide) -> Self {
        FontraGuideline {
            name: val.name.clone(),
            x: val.pos.x,
            y: val.pos.y,
            angle: val.pos.angle,
            locked: false,
        }
    }
}

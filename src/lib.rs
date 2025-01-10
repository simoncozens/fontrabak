#![allow(non_snake_case)]
use babelfont::convertors::fontra;
use serde::Serialize;
use serde_json::json;
use serde_wasm_bindgen::Serializer;
use std::collections::HashMap;
use wasm_bindgen::{prelude::*, JsValue};
use web_sys::Blob;

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

#[wasm_bindgen]
pub struct Font(babelfont::Font);

#[wasm_bindgen]
impl Font {
    #[wasm_bindgen(constructor)]
    pub fn new(font_a: &JsValue) -> Result<Font, JsValue> {
        let font = if font_a.is_falsy() {
            babelfont::Font::new()
        } else {
            babelfont::convertors::glyphs3::load_str(
                &font_a
                    .as_string()
                    .ok_or(JsValue::from_str("Font must be a string or null"))?,
                "".into(),
            )
            .map_err(|e| JsValue::from_str(&format!("Error loading font: {:?}", e)))?
        };
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
        let axes = self.0.as_fontra_axes();
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
        let fontraglyph = self
            .0
            .get_fontra_glyph(&name)
            .ok_or_else(|| JsValue::from_str("Glyph not found"))?;
        serialize_json_compatible(&fontraglyph)
    }

    pub fn getSources(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        let sources: Vec<fontra::Source> = self.0.masters.iter().map(Into::into).collect();
        serialize_json_compatible(&sources)
    }

    pub fn getUnitsPerEm(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serialize_json_compatible(&self.0.upm)
    }

    pub fn isReadOnly(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serialize_json_compatible(&false)
    }

    pub fn getBackEndInfo(&self) -> JsValue {
        let info = fontra::BackendInfo {
            features: vec![],
            project_manager_features: json!({
                "export-as": ["glyphs"]
            }),
        };
        serialize_json_compatible(&info).unwrap()
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

    pub fn exportAs(&self, _options: JsValue) -> Option<web_sys::Blob> {
        let plist = self.0.as_glyphslib().to_string().ok()?;
        let bytes = plist.as_bytes();
        let uint8arr = js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(bytes) }.into());
        let array = js_sys::Array::new();
        array.push(&uint8arr.buffer());
        let bag = web_sys::BlobPropertyBag::new();
        bag.set_type("text/plain");
        Blob::new_with_u8_array_sequence_and_options(&array, &bag).ok()
    }

    pub fn findGlyphsThatUseGlyph(&self, _glyphname: String) -> Vec<String> {
        vec![]
    }
    pub fn on(&self, _event: String, _callback: JsValue) -> JsValue {
        JsValue::NULL
    }

    pub fn getFontInfo(&self) -> JsValue {
        let fontinfo = self.0.as_fontra_info();
        serialize_json_compatible(&fontinfo).unwrap()
    }
}

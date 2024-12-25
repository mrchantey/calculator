use wasm_bindgen::prelude::*;

pub trait JsValueSerdeExt {
    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    fn from_serde<T>(t: &T) -> serde_json::Result<JsValue>
    where
        T: serde::ser::Serialize + ?Sized;

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    #[allow(clippy::wrong_self_convention)]
    fn into_serde<T>(&self) -> serde_json::Result<T>
    where
        T: for<'a> serde::de::Deserialize<'a>;
}

impl JsValueSerdeExt for JsValue {
    fn from_serde<T>(t: &T) -> serde_json::Result<JsValue>
    where
        T: serde::ser::Serialize + ?Sized,
    {
        let s = serde_json::to_string(t)?;
        Ok(js_sys::JSON::parse(&s).unwrap_throw())
    }

    fn into_serde<T>(&self) -> serde_json::Result<T>
    where
        T: for<'a> serde::de::Deserialize<'a>,
    {
        // Turns out `JSON.stringify(undefined) === undefined`, so if
        // we're passed `undefined` reinterpret it as `null` for JSON
        // purposes.
        let s = if self.is_undefined() {
            String::from("null")
        } else {
            js_sys::JSON::stringify(self)
                .map(String::from)
                .unwrap_throw()
        };
        serde_json::from_str(&s)
    }
}

// Copyright 2022 Jeremy Wall (Jeremy@marzhilsltudios.com)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    pub fn mark(a: &str);
    #[wasm_bindgen(catch, js_namespace = performance)]
    pub fn measure(name: String, startMark: String) -> Result<(), JsValue>;

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn error(message: String);
    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    pub fn warn(message: String);
    #[wasm_bindgen(js_namespace = console, js_name = info)]
    pub fn info(message: String);
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    pub fn debug(message: String);
    #[wasm_bindgen(js_namespace = console, js_name = trace)]
    pub fn trace(message: String);
}

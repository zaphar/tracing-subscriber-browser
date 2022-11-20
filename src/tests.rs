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
use tracing::{info, instrument};
use wasm_bindgen_test::wasm_bindgen_test;

use super::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[instrument]
fn instrumented_incrementor(arg: i32) -> i32 {
    info!("Incrementing arg by 1");
    arg + 1
}

#[instrument]
fn instrumented_function(arg: i32) {
    info!("Calling increment function");
    let incremented = instrumented_incrementor(arg);
    info!(incremented, "Finished incrementing function");
}

#[instrument]
#[wasm_bindgen_test]
fn it_works() {
    configure_as_global_default();
    info!("Calling instrumented function");
    instrumented_function(3);
}

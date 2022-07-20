<!--
 Copyright 2022 Jeremy Wall (Jeremy@marzhilsltudios.com)
 
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at
 
     http://www.apache.org/licenses/LICENSE-2.0
 
 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
-->

# tracing-browser-subscriber

This implements a minimal [tracing](https://crates.io/crates/tracing) subscriber for use in browser/webassembly.
It allows you to use tracing in the browser to log as well as record timings for spans. At the time of writing it 
differs from [tracing-wasm](https://crates.io/crates/tracing) in the following ways.

* It does not support colors in the log output.
* It does not mark or measure events in the browser performance data only spans.
* It use the console `error`, `warn`, `info`, `debug`, and `trace` mechanism to record logs.

If the above features/restrictions do not appeal to you you may want to use `tracing-wasm` instead.

## Example

```rust
use tracing_browser_subscriber;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
fn main() {
    tracing_browser_subscriber::configure_as_global_default();
    // Rest of our webassembly code goes here
}
```
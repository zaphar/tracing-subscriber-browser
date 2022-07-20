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
use tracing::field::{Field, Visit};

use core::fmt::Display;
use core::fmt::Write;

pub struct Recorder {
    buffer: String,
}

impl core::default::Default for Recorder {
    fn default() -> Self {
        Self {
            buffer: String::new(),
        }
    }
}

impl Recorder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Visit for Recorder {
    fn record_debug(&mut self, field: &Field, value: &dyn core::fmt::Debug) {
        if field.name() == "message" {
            // NOTE(jwall): We always want the message to appear at the front of the
            // log content.
            if !self.buffer.is_empty() {
                self.buffer = format!("{:?}\n{}", value, self.buffer);
            } else {
                self.buffer = format!("{:?}\n", value);
            }
        } else {
            writeln!(self.buffer, "{}={:?} ", field.name(), value)
                .expect("Failed to write field to ConsoleRecorder buffer");
        }
    }
}

impl Display for Recorder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buffer)
    }
}

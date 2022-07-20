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
use core::default::Default;
use core::fmt::Write;

use tracing;
use tracing::span;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::registry::{LookupSpan, Registry};

mod console;
use console::Recorder;
use wasm_bindgen::UnwrapThrowExt;

mod typings;

pub struct BrowserLayer {
    record_timings: bool,
    max_level: tracing::Level,
}

impl Default for BrowserLayer {
    fn default() -> Self {
        Self {
            record_timings: true,
            max_level: tracing::Level::INFO,
        }
    }
}

impl BrowserLayer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_level(mut self, level: tracing::Level) -> Self {
        self.max_level = level;
        self
    }

    pub fn with_record_timings(mut self, record_timings: bool) -> Self {
        self.record_timings = record_timings;
        self
    }

    pub fn write_for_level(&self, level: &tracing::Level, origin: String, recorder: Recorder) {
        let f: fn(String) = match *level {
            tracing::Level::ERROR => typings::error,
            tracing::Level::WARN => typings::warn,
            tracing::Level::INFO => typings::info,
            tracing::Level::DEBUG => typings::debug,
            tracing::Level::TRACE => typings::trace,
        };
        f(format!("{} {} {}", level, origin, recorder));
    }
}

fn format_mark(name: Option<&str>, id: &tracing::Id) -> String {
    let mut buffer = String::new();
    if let Some(n) = name {
        write!(buffer, "[{}]", n).unwrap_throw();
    }
    write!(buffer, "{:x}", id.into_u64()).unwrap_throw();
    buffer
}

impl<S: tracing::Subscriber + for<'a> LookupSpan<'a>> Layer<S> for BrowserLayer {
    fn enabled(&self, metadata: &tracing::Metadata<'_>, _ctx: Context<'_, S>) -> bool {
        metadata.level() <= &self.max_level
    }

    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let mut recorder = Recorder::new();
        attrs.record(&mut recorder);
        if let Some(span_ref) = ctx.span(id) {
            span_ref.extensions_mut().insert(recorder);
        }
    }

    fn on_record(&self, span: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>) {
        if let Some(span_ref) = ctx.span(span) {
            if let Some(recorder) = span_ref.extensions_mut().get_mut::<Recorder>() {
                values.record(recorder)
            }
        }
    }

    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        let mut recorder = Recorder::new();
        event.record(&mut recorder);
        let metadata = event.metadata();
        self.write_for_level(
            metadata.level(),
            metadata
                .file()
                .and_then(|file| metadata.line().map(|ln| format!("{}:{}", file, ln)))
                .unwrap_or_default(),
            recorder,
        );
    }

    fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
        if self.record_timings {
            if let Some(span_ref) = ctx.span(id) {
                typings::mark(&format_mark(Some(span_ref.metadata().name()), id));
            } else {
                typings::mark(&format_mark(None, id));
            }
        }
    }

    fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>) {
        if self.record_timings {
            if let Some(span_ref) = ctx.span(id) {
                let metadata = span_ref.metadata();
                if let Some(recorder) = span_ref.extensions_mut().get_mut::<Recorder>() {
                    typings::measure(
                        format!(
                            "[{}] {} {}",
                            metadata.name(),
                            metadata.module_path().unwrap_or("..."),
                            recorder
                        ),
                        format_mark(Some(span_ref.metadata().name()), id),
                    )
                    .unwrap_throw();
                } else {
                    typings::measure(
                        format!(
                            "[{}] {}",
                            metadata.name(),
                            metadata.module_path().unwrap_or("...")
                        ),
                        format_mark(None, id),
                    )
                    .unwrap_throw();
                }
            }
        }
    }
}

pub fn configure_as_global_default() {
    tracing::subscriber::set_global_default(
        Registry::default().with(BrowserLayer::new().with_max_level(tracing::Level::DEBUG)),
    )
    .expect("default global");
}
#[cfg(test)]
mod tests;

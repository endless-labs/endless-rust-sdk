// Copyright © Endless
// Copyright © Aptos Foundation

// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use endless_infallible::RwLock;
use endless_logger::{endless_logger::EndlessData, info, Writer};
use std::sync::Arc;

#[derive(Default)]
struct VecWriter {
    logs: Arc<RwLock<Vec<String>>>,
}

impl Writer for VecWriter {
    fn write(&self, log: String) {
        self.logs.write().push(log)
    }

    fn write_buferred(&mut self, log: String) {
        self.write(log);
    }
}

#[test]
fn test_custom_formatter() {
    let writer = VecWriter::default();
    let logs = writer.logs.clone();
    EndlessData::builder()
        .is_async(false)
        .printer(Box::new(writer))
        .custom_format(|entry| {
            use std::fmt::Write;
            let mut w = String::new();
            write!(w, "0000-00-00")?;
            write!(w, " [{}]", entry.metadata().level())?;
            if let Some(message) = entry.message() {
                write!(w, " {}", message)?;
            }
            if !entry.data().is_empty() {
                write!(w, " {}", serde_json::to_string(&entry.data()).unwrap())?;
            }
            Ok(w)
        })
        .build();

    assert_eq!(logs.read().len(), 0);
    info!("Hello");
    assert_eq!(logs.read().len(), 1);
    let string = logs.write().remove(0);
    assert_eq!(string, "0000-00-00 [INFO] Hello");
}

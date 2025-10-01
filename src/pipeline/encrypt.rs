use std::fs;
use std::sync::mpsc::Sender;
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::pipeline::{PipelineContext, PipelineMessage};
use crate::pipeline::step::PipelineStep;

use goblin::Object;

/// Minimum length of ASCII string to consider (tuneable)
const MIN_STRING_LEN: usize = 4;
/// XOR key for the simple encryption simulation
const XOR_KEY: u8 = 0xAA;

pub struct EncryptStringsStep;

impl EncryptStringsStep {
    pub fn new() -> Self {
        Self {}
    }
}

/// Helper: returns true if byte is printable ASCII (space..~)
fn is_printable_ascii(b: u8) -> bool {
    b >= 0x20 && b <= 0x7E
}

impl PipelineStep for EncryptStringsStep {
    fn run(&self, ctx: &mut PipelineContext, tx: &Sender<PipelineMessage>) -> anyhow::Result<()> {
        tx.send(PipelineMessage::Log("Encrypting strings step (PoC) started".into())).ok();
        tx.send(PipelineMessage::Progress(0.15)).ok();
        std::thread::sleep(Duration::from_millis(120));

        // Read file bytes
        let bytes = fs::read(&ctx.input_path).map_err(|e| {
            anyhow::anyhow!("Encrypt step: failed to read '{}': {}", ctx.input_path, e)
        })?;

        // Parse as PE to get sections; fall back to scanning whole file if not PE
        let mut candidate_ranges: Vec<(usize, usize)> = Vec::new(); // (start, length)

        match Object::parse(&bytes) {
            Ok(Object::PE(pe)) => {
                for sec in &pe.sections {
                    let start = sec.pointer_to_raw_data as usize;
                    let size = sec.size_of_raw_data as usize;
                    if start == 0 || size == 0 {
                        continue;
                    }
                    let end = start.saturating_add(size).min(bytes.len());
                    if end > start {
                        candidate_ranges.push((start, end - start));
                    }
                }
                if candidate_ranges.is_empty() {
                    candidate_ranges.push((0, bytes.len()));
                }
                tx.send(PipelineMessage::Log(format!(
                    "Encrypt step: scanning {} sections for strings (PoC)",
                    candidate_ranges.len()
                )))
                .ok();
            }
            Ok(_) | Err(_) => {
                candidate_ranges.push((0, bytes.len()));
                tx.send(PipelineMessage::Log(
                    "Encrypt step: file not recognized as PE; scanning whole file (PoC)".into(),
                ))
                .ok();
            }
        }

        // Find ASCII strings in the candidate ranges
        let mut found_strings: Vec<(usize, usize)> = Vec::new();
        let mut total_checked = 0usize;
        for (ri, (start, len)) in candidate_ranges.iter().cloned().enumerate() {
            // verificar cancelamento
            if ctx.cancel_flag.load(Ordering::Relaxed) {
                let _ = tx.send(PipelineMessage::Cancelled);
                return Ok(());
            }
            let slice = &bytes[start..start + len];
            let mut i = 0usize;
            while i < slice.len() {
                if !is_printable_ascii(slice[i]) {
                    i += 1;
                    continue;
                }
                let run_start = i;
                while i < slice.len() && is_printable_ascii(slice[i]) {
                    i += 1;
                }
                let run_len = i - run_start;
                if run_len >= MIN_STRING_LEN {
                    found_strings.push((start + run_start, run_len));
                }
                total_checked += run_len;
            }

            let sec_progress = 0.15 + (ri as f32 + 1.0) / (candidate_ranges.len() as f32) * 0.20;
            tx.send(PipelineMessage::Progress(sec_progress.min(0.4))).ok();
            std::thread::sleep(Duration::from_millis(80));
        }

        // Summary log
        let count = found_strings.len();
        tx.send(PipelineMessage::Log(format!(
            "Found {} candidate strings (min length = {}) across {} bytes scanned (PoC)",
            count, MIN_STRING_LEN, total_checked
        )))
        .ok();

        // Simulate encryption
        if count > 0 {
            let mut out_bytes = bytes.clone();
            for (idx, (off, len)) in found_strings.iter().enumerate() {
                // verificar cancelamento
                if ctx.cancel_flag.load(Ordering::Relaxed) {
                    let _ = tx.send(PipelineMessage::Cancelled);
                    return Ok(());
                }
                let end = (*off).saturating_add(*len).min(out_bytes.len());
                for b in out_bytes[*off..end].iter_mut() {
                    *b ^= XOR_KEY;
                }
                let p = 0.4 + (idx as f32 + 1.0) / (count as f32) * 0.4;
                tx.send(PipelineMessage::Progress(p.min(0.85))).ok();
            }

            let out_path = format!("{}.enc", ctx.input_path);
            match fs::write(&out_path, &out_bytes) {
                Ok(_) => {
                    tx.send(PipelineMessage::Log(format!(
                        "Wrote PoC encrypted file: {}",
                        out_path
                    )))
                    .ok();
                }
                Err(e) => {
                    tx.send(PipelineMessage::Log(format!(
                        "Failed to write PoC encrypted file: {}",
                        e
                    )))
                    .ok();
                }
            }
        } else {
            tx.send(PipelineMessage::Log(
                "No candidate strings found; skipping encryption step (PoC)".into(),
            ))
            .ok();
        }

        tx.send(PipelineMessage::Progress(0.85)).ok();
        std::thread::sleep(Duration::from_millis(120));
        tx.send(PipelineMessage::Log("Encrypt strings step (PoC) completed".into())).ok();

        Ok(())
    }
}

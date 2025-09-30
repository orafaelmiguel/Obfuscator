use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::pipeline::{PipelineContext, PipelineMessage};
use crate::pipeline::step::PipelineStep;

use goblin::Object;

pub struct ParseStep;

impl ParseStep {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipelineStep for ParseStep {
    fn run(&self, ctx: &PipelineContext, tx: &Sender<PipelineMessage>) {
        // log start
        let _ = tx.send(PipelineMessage::Log(format!("Parsing file: {}", ctx.input_path)));

        // small simulated progress slices while we do work
        let slices = 3;
        for i in 0..slices {
            std::thread::sleep(Duration::from_millis(120));
            let progress = (i as f32 + 1.0) / (slices as f32) * 0.10; // small share
            let _ = tx.send(PipelineMessage::Progress(progress));
        }

        // Try to read the file
        let path = Path::new(&ctx.input_path);
        let bytes = match fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                let _ = tx.send(PipelineMessage::Error(format!(
                    "Failed to read file '{}': {}",
                    ctx.input_path, e
                )));
                return;
            }
        };

        // Try to parse using goblin::Object
        match Object::parse(&bytes) {
            Ok(Object::PE(pe)) => {
                // architecture / machine
                let machine = pe.header.coff_header.machine;
                let arch = match machine {
                    0x14c => "x86 (32-bit)",
                    0x8664 => "x86_64 (64-bit)",
                    m => &format!("unknown (0x{:x})", m),
                };
                let _ = tx.send(PipelineMessage::Log(format!("Detected PE: {}", arch)));

                // sections count and names
                let sections = pe.sections.len();
                let _ = tx.send(PipelineMessage::Log(format!("Sections: {}", sections)));

                // list section names (safe conversion)
                let mut names = Vec::new();
                for sec in &pe.sections {
                    if let Ok(name) = std::str::from_utf8(&sec.name) {
                        names.push(name.trim_end_matches(char::from(0)).to_string());
                    } else {
                        names.push(String::from("<non-utf8>"));
                    }
                }
                if !names.is_empty() {
                    let _ = tx.send(PipelineMessage::Log(format!(
                        "Section names: {}",
                        names.join(", ")
                    )));
                }

                // imports summary
                let import_count = pe.imports.len();
                let mut import_names = Vec::new();
                for imp in &pe.imports {
                    import_names.push(imp.dll.to_string());
                }
                let _ = tx.send(PipelineMessage::Log(format!(
                    "Import DLLs: {} ({} entries)",
                    if import_names.is_empty() { "<none>".to_string() } else { import_names.join(", ") },
                    import_count
                )));

                // exports summary
                let export_count = pe.exports.len();
                if export_count > 0 {
                    let _ = tx.send(PipelineMessage::Log(format!("Exports: {} entries", export_count)));
                } else {
                    let _ = tx.send(PipelineMessage::Log("Exports: none detected".into()));
                }

                // finished parsing step (progress bump)
                let _ = tx.send(PipelineMessage::Progress(0.20));
                let _ = tx.send(PipelineMessage::Log(format!(
                    "Parsing complete: {} sections found",
                    sections
                )));
            }
            Ok(other) => {
                // Not PE
                let _ = tx.send(PipelineMessage::Error(format!(
                    "File is not a PE executable (detected: {:?})",
                    other
                )));
            }
            Err(e) => {
                let _ = tx.send(PipelineMessage::Error(format!(
                    "Failed to parse file '{}': {}",
                    ctx.input_path, e
                )));
            }
        }
    }
}

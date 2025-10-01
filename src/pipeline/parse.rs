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
    fn run(&self, ctx: &mut PipelineContext, tx: &Sender<PipelineMessage>) -> anyhow::Result<()> {
        // log start
        tx.send(PipelineMessage::Log(format!("Parsing file: {}", ctx.input_path))).ok();

        // simulated progress
        let slices = 3;
        for i in 0..slices {
            std::thread::sleep(Duration::from_millis(120));
            let progress = (i as f32 + 1.0) / (slices as f32) * 0.10;
            tx.send(PipelineMessage::Progress(progress)).ok();
        }

        // Try to read the file
        let path = Path::new(&ctx.input_path);
        let bytes = fs::read(path).map_err(|e| {
            anyhow::anyhow!("Failed to read file '{}': {}", ctx.input_path, e)
        })?;

        // Try to parse using goblin::Object
        match Object::parse(&bytes) {
            Ok(Object::PE(pe)) => {
                let machine = pe.header.coff_header.machine;
                let arch = match machine {
                    0x14c => "x86 (32-bit)".to_string(),
                    0x8664 => "x86_64 (64-bit)".to_string(),
                    m => format!("unknown (0x{:x})", m),
                };
                tx.send(PipelineMessage::Log(format!("Detected PE: {}", arch))).ok();

                let sections = pe.sections.len();
                tx.send(PipelineMessage::Log(format!("Sections: {}", sections))).ok();

                let mut names = Vec::new();
                for sec in &pe.sections {
                    if let Ok(name) = std::str::from_utf8(&sec.name) {
                        names.push(name.trim_end_matches(char::from(0)).to_string());
                    } else {
                        names.push("<non-utf8>".into());
                    }
                }
                if !names.is_empty() {
                    tx.send(PipelineMessage::Log(format!(
                        "Section names: {}",
                        names.join(", ")
                    )))
                    .ok();
                }

                let import_count = pe.imports.len();
                let import_names: Vec<String> =
                    pe.imports.iter().map(|imp| imp.dll.to_string()).collect();
                tx.send(PipelineMessage::Log(format!(
                    "Import DLLs: {} ({} entries)",
                    if import_names.is_empty() {
                        "<none>".to_string()
                    } else {
                        import_names.join(", ")
                    },
                    import_count
                )))
                .ok();

                let export_count = pe.exports.len();
                if export_count > 0 {
                    tx.send(PipelineMessage::Log(format!(
                        "Exports: {} entries",
                        export_count
                    )))
                    .ok();
                } else {
                    tx.send(PipelineMessage::Log("Exports: none detected".into())).ok();
                }

                tx.send(PipelineMessage::Progress(0.20)).ok();
                tx.send(PipelineMessage::Log(format!(
                    "Parsing complete: {} sections found",
                    sections
                )))
                .ok();
            }
            Ok(other) => {
                tx.send(PipelineMessage::Error(format!(
                    "File is not a PE executable (detected: {:?})",
                    other
                )))
                .ok();
            }
            Err(e) => {
                tx.send(PipelineMessage::Error(format!(
                    "Failed to parse file '{}': {}",
                    ctx.input_path, e
                )))
                .ok();
            }
        }

        Ok(())
    }
}

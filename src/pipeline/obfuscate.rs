use std::fs;
use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::pipeline::{PipelineContext, PipelineMessage};
use crate::pipeline::step::PipelineStep;

use goblin::Object;

/// Mock initial obfuscation step.
/// - Detects "functions" by looking at exports when available (fallback: simulated count)
/// - Produces a fake renaming mapping and writes `<input>.obf-map` as proof-of-concept
pub struct ObfuscateFunctionsStep;

impl ObfuscateFunctionsStep {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipelineStep for ObfuscateFunctionsStep {
    fn run(&self, ctx: &mut PipelineContext, tx: &Sender<PipelineMessage>) -> anyhow::Result<()> {
        tx.send(PipelineMessage::Log("Obfuscation step (mock) started".into())).ok();
        tx.send(PipelineMessage::Progress(0.45)).ok();
        std::thread::sleep(Duration::from_millis(160));

        // read file bytes
        let bytes = fs::read(&ctx.input_path).map_err(|e| {
            anyhow::anyhow!("Obfuscation step: failed to read '{}': {}", ctx.input_path, e)
        })?;

        // attempt to detect functions: prefer exports count as a naive proxy
        let mut function_names: Vec<String> = Vec::new();

        match Object::parse(&bytes) {
            Ok(Object::PE(pe)) => {
                if !pe.exports.is_empty() {
                    for (i, exp) in pe.exports.iter().enumerate() {
                        let name = exp
                            .name
                            .as_ref()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| format!("export_{}", i));
                        function_names.push(name);
                    }
                    tx.send(PipelineMessage::Log(format!(
                        "Obfuscation: detected {} exported functions (using exports as proxy)",
                        function_names.len()
                    )))
                    .ok();
                } else {
                    let approx = (pe.sections.len() * 2).saturating_sub(1);
                    for i in 0..approx {
                        function_names.push(format!("func_approx_{}", i + 1));
                    }
                    tx.send(PipelineMessage::Log(format!(
                        "Obfuscation: no exports found; using heuristic => {} functions (mock)",
                        function_names.len()
                    )))
                    .ok();
                }
            }
            Ok(_) | Err(_) => {
                let simulated = 8usize;
                for i in 0..simulated {
                    function_names.push(format!("sim_func_{}", i + 1));
                }
                tx.send(PipelineMessage::Log(format!(
                    "Obfuscation: file not parsed as PE; simulating {} functions (mock)",
                    simulated
                )))
                .ok();
            }
        }

        // Simulate renaming: generate mapping old -> new
        let total = function_names.len();
        let mut mapping_lines: Vec<String> = Vec::with_capacity(total);
        for (i, old) in function_names.iter().enumerate() {
            let new = format!("f_{:04}", i + 1);
            mapping_lines.push(format!("{} => {}", old, new));

            let p = 0.45 + (i as f32 + 1.0) / (total.max(1) as f32) * 0.25;
            tx.send(PipelineMessage::Progress(p.min(0.75))).ok();
            std::thread::sleep(Duration::from_millis(80));
        }

        // Write mapping file next to input (PoC)
        let map_path = format!("{}.obf-map", ctx.input_path);
        match fs::write(&map_path, mapping_lines.join("\n")) {
            Ok(_) => {
                tx.send(PipelineMessage::Log(format!(
                    "Obfuscation (mock): wrote mapping file {} ({} entries)",
                    map_path, total
                )))
                .ok();
            }
            Err(e) => {
                tx.send(PipelineMessage::Log(format!(
                    "Obfuscation (mock): failed to write mapping file: {}",
                    e
                )))
                .ok();
            }
        }

        tx.send(PipelineMessage::Log(format!("Obfuscated {} functions (mock)", total))).ok();
        tx.send(PipelineMessage::Progress(0.75)).ok();
        std::thread::sleep(Duration::from_millis(120));
        tx.send(PipelineMessage::Log("Obfuscation step (mock) completed".into())).ok();

        Ok(())
    }
}

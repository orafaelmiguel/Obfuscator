use std::sync::mpsc;
use std::thread;

pub mod parse;
pub mod encrypt;
pub mod obfuscate;
pub mod write;
pub mod step;

use step::PipelineStep;
use parse::ParseStep;
use encrypt::EncryptStringsStep;
use obfuscate::ObfuscateFunctionsStep;
use write::WriteOutputStep;

#[derive(Debug, Clone)]
pub enum PipelineMessage {
    Log(String),
    Progress(f32),
    Done(String),   // output file path (string)
    Error(String),
}

/// Context passed to steps
#[derive(Clone, Debug)]
pub struct PipelineContext {
    pub input_path: String,
    pub encrypt_strings: bool,
    pub obfuscate_functions: bool,
    // Futuro: output_dir, auth_token, config avançada etc.
}

impl PipelineContext {
    pub fn new(input_path: impl Into<String>, encrypt: bool, obfuscate: bool) -> Self {
        Self {
            input_path: input_path.into(),
            encrypt_strings: encrypt,
            obfuscate_functions: obfuscate,
        }
    }
}

/// Starts the modular pipeline in a background thread and returns the Receiver.
/// The caller (UI) should store the Receiver in state and poll it each frame.
pub fn start_pipeline(ctx: PipelineContext) -> mpsc::Receiver<PipelineMessage> {
    let (tx, rx) = mpsc::channel::<PipelineMessage>();

    // Define ordem das etapas dinamicamente
    let mut steps: Vec<Box<dyn PipelineStep + Send>> = Vec::new();
    steps.push(Box::new(ParseStep::new()));
    if ctx.encrypt_strings {
        steps.push(Box::new(EncryptStringsStep::new()));
    }
    if ctx.obfuscate_functions {
        steps.push(Box::new(ObfuscateFunctionsStep::new()));
    }
    steps.push(Box::new(WriteOutputStep::new()));

    thread::spawn(move || {
        let _ = tx.send(PipelineMessage::Log(format!(
            "Pipeline started for '{}'",
            ctx.input_path
        )));
        let total = steps.len() as f32;

        for (i, step) in steps.into_iter().enumerate() {
            let base = i as f32 / total;
            let step_share = 1.0 / total;

            // Executa o passo (cada passo envia seus próprios logs e progresso)
            step.run(&ctx, &tx);

            // Marca progresso após passo concluído
            let _ = tx.send(PipelineMessage::Progress((base + step_share * 0.95).min(1.0)));
        }

        // Gera saída mock
        let output = format!("{}.obscura-protected", ctx.input_path);
        let _ = std::fs::write(&output, "Obscura Defender - mock protected file");

        let _ = tx.send(PipelineMessage::Log(format!("Pipeline finished, output: {}", &output)));
        let _ = tx.send(PipelineMessage::Progress(1.0));
        let _ = tx.send(PipelineMessage::Done(output));
    });

    rx
}

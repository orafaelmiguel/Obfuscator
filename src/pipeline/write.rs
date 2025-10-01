use super::{PipelineContext, PipelineMessage, step::PipelineStep};
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

pub struct WriteOutputStep;

impl PipelineStep for WriteOutputStep {
    fn run(&self, ctx: &mut PipelineContext, tx: &Sender<PipelineMessage>) -> anyhow::Result<()> {
        tx.send(PipelineMessage::Log("Writing output file...".into())).ok();

        let input_path = PathBuf::from(&ctx.input_path);
        let output_path = input_path.with_extension("obscura-protected.exe");

        // For now: just copy the original file as placeholder
        fs::copy(&ctx.input_path, &output_path)?;

        tx.send(PipelineMessage::Log(format!(
            "Output written to {}",
            output_path.display()
        )))
        .ok();

        // Notify pipeline completion with the final path
        tx.send(PipelineMessage::Done(output_path.to_string_lossy().to_string()))
            .ok();

        Ok(())
    }
}

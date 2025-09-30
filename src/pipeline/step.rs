use std::sync::mpsc::Sender;
use crate::pipeline::{PipelineContext, PipelineMessage};

/// Trait that each pipeline step implements.
pub trait PipelineStep {
    /// Execute the step. Should send `PipelineMessage` items through `tx`.
    fn run(&self, ctx: &PipelineContext, tx: &Sender<PipelineMessage>);
}

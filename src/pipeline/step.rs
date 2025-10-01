use std::sync::mpsc::Sender;
use crate::pipeline::PipelineMessage;

pub trait PipelineStep: Send {
    fn run(&self, ctx: &mut super::PipelineContext, tx: &Sender<PipelineMessage>) -> anyhow::Result<()>;
}
use anyhow::Result;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::cli::Args;

pub trait Widget {
    fn update(&mut self, args: &Args) -> Result<()>;
    fn draw(&self, f: &mut Frame, args: &Args, s: Rect) -> Result<()>;
}
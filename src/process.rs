use psutil::process::*;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::cli::Args;
use crate::widget::Widget;

pub struct ProcessWidget {
    title: String
}

impl ProcessWidget {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string()
        }
    }
}

impl Widget for ProcessWidget {
    fn update(&mut self, args: &Args) -> anyhow::Result<()> {
        todo!()
    }

    fn draw(&self, f: &mut Frame, args: &Args, s: Rect) -> anyhow::Result<()> {
        todo!()
    }
}
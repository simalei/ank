use ratatui::Frame;
use ratatui::prelude::*;
use crate::App;
use crate::widget::Widget;

pub fn ui(frame: &mut Frame, app: &mut App) -> anyhow::Result<()> {
    let size = frame.size();
    let layout = Layout::new(Direction::Vertical,
                             [Constraint::Percentage(50), Constraint::Percentage(50)]
    ).split(size);

    app.cpu.draw(frame, &app.args, layout[0])?;
    app.disk.draw(frame, &app.args, layout[1])?;

    Ok(())


}
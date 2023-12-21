use ratatui::Frame;
use ratatui::prelude::*;
use crate::App;
use crate::widget::Widget;

pub fn ui(frame: &mut Frame, app: &mut App) -> anyhow::Result<()> {
    let size = frame.size();

    let layout = Layout::new(Direction::Horizontal, [
        Constraint::Percentage(50), Constraint::Percentage(50)
    ]).split(size);

    let left_side = Layout::new(Direction::Vertical, [
        Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)
    ]).split(layout[0]);

    let right_side = Layout::new(Direction::Vertical, [
        Constraint::Percentage(50), Constraint::Percentage(50)
    ]).split(layout[1]);

    app.cpu.draw(frame, &app.args, right_side[0])?;
    app.disk.draw(frame, &app.args, right_side[1])?;
    app.memory.draw(frame, &app.args, left_side[0])?;

    Ok(())


}
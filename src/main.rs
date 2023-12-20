mod cpu;
mod widget;
mod cli;
mod disk;
mod ui;
mod event;
mod memory;

use std::io::stdout;
use std::time::Duration;
use crossterm::{
    event::{Event},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{prelude::*};

use crossbeam_channel::{Receiver, select, unbounded};

use anyhow::Result;

use better_panic;
use clap::Parser;


use crate::cpu::CPUWidget;
use crate::widget::Widget;
use crate::cli::Args;
use crate::disk::DiskWidget;
use crate::ui::ui;
use crate::event::handle_events;

struct App {
    quit: bool,
    args: Args,
    cpu: CPUWidget,
    disk: DiskWidget
}


impl App {
    fn new() -> Self {

        Self {
            quit: false,
            args: Args::parse(),
            cpu: CPUWidget::new(" CPU Usage "),
            disk: DiskWidget::new(),
        }
    }
}

fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
        better_panic::Settings::auto().create_panic_handler()(info);
    }));
}


fn event_thread() -> Receiver<Event> {
    let (tx, rx) = unbounded();
    std::thread::spawn(move || loop {
        tx.send(crossterm::event::read().unwrap()).unwrap();
    });
    rx
}

fn update_thread() -> Receiver<()> {
    let (tx, rx) = unbounded();
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(1000));
        tx.send(()).unwrap();
    });
    rx
}



fn main() -> Result<()> {
    setup_panic_hook();

    let mut app = App::new();

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let update_recv = update_thread();
    let event_recv = event_thread();



    //app.cpu.update(&app.args)?;
    app.disk.update(&app.args)?;

    terminal.draw(|f| { ui(f, &mut app).unwrap(); })?;

    loop {
        if app.quit {
            break;
        }

        select! {
            recv(update_recv) -> _ => {
                app.cpu.update(&app.args)?;
                //app.disk.update(&app.args)?;
            }
            recv(event_recv) -> event => {
                handle_events(&mut app, event.unwrap());
            }
        }
        terminal.draw(|f| { ui(f, &mut app).unwrap(); })?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

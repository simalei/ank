use std::cmp::max;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::cli::Args;
use crate::widget::Widget;

use ratatui::prelude::*;
use psutil::memory::*;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, LegendPosition};
use crate::{mb, mb_or_gb, mb_or_gb_label};

#[derive(Default)]
struct Memory {
    total: u64,
    used: u64,
    percent: f64,
    data: Vec<(f64, f64)>
}


pub struct MemoryWidget {
    title: String,
    main_memory: Memory,
    swap_memory: Option<Memory>,
    update_count: f64
}

impl MemoryWidget {
    pub fn new(title: &str) -> Self {

        let swap_memory: Option<Memory> = match swap_memory() {
            Ok(swap_memory) => {
                let memory = Memory {
                    total: swap_memory.total(),
                    used: swap_memory.used(),
                    percent: swap_memory.percent() as f64,
                    data: vec![]
                };
                Some(memory)
            }
            Err(_) => { None }
        };
        
        Self {
            title: title.to_string(),
            main_memory: Memory::default(),
            swap_memory,
            update_count: 0.0
        }
    }
}


impl Widget for MemoryWidget {
    fn update(&mut self, args: &Args) -> anyhow::Result<()> {

        self.update_count += 1.0;

        self.main_memory.used = virtual_memory().unwrap().used();
        self.main_memory.total = virtual_memory().unwrap().total();
        self.main_memory.percent = virtual_memory().unwrap().percent() as f64;
        self.main_memory.data.push((self.update_count, self.main_memory.used as f64));


        match &mut self.swap_memory {
            Some(swap_memory) => {
                use psutil::memory::swap_memory as swap_fun;
                swap_memory.used = swap_fun().unwrap().used();
                swap_memory.total = swap_fun().unwrap().total();
                swap_memory.percent = swap_fun().unwrap().percent() as f64;
                swap_memory.data.push((self.update_count, swap_fun().unwrap().used() as f64))
            }
            _ => {}
        }

        Ok(())
    }

    fn draw(&self, f: &mut Frame, args: &Args, s: Rect) -> anyhow::Result<()> {
        let size_label = mb_or_gb_label!(args);

        let mut datasets = vec![
            Dataset::default()
                .name(format!("Virtual memory • Total: {} {size_label} • Used: {} {size_label} ({:.1}%)",
                              mb_or_gb!(args, self.main_memory.total),
                              mb_or_gb!(args, self.main_memory.used),
                              self.main_memory.percent))
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::Red))
                .graph_type(GraphType::Line)
                .data(&self.main_memory.data)
        ];

        if self.swap_memory.is_some() {
            datasets.push(
                Dataset::default()
                    .name(format!("Swap memory • Total: {} {size_label} • Used: {} {size_label} ({:.1}%)",
                                  mb_or_gb!(args, self.swap_memory.as_ref().unwrap().total),
                                  mb_or_gb!(args, self.swap_memory.as_ref().unwrap().used),
                                  self.swap_memory.as_ref().unwrap().percent
                    ))
                    .marker(Marker::Braille)
                    .style(Style::default().fg(Color::Blue))
                    .graph_type(GraphType::Line)
                    .data(&self.swap_memory.as_ref().unwrap().data)
            );
        }

        let chart = Chart::new(datasets)
            .legend_position(Some(LegendPosition::TopLeft))
            .hidden_legend_constraints((Constraint::Min(0), Constraint::Min(0))) // Always show the legend. Maybe will be changed later
            .block(Block::new().title(&*self.title).borders(Borders::ALL))
            .x_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds([self.update_count - 25.0, self.update_count + 1.0])
            )
            .y_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, self.main_memory.total as f64])
            );

        f.render_widget(chart, s);

        Ok(())
    }
}
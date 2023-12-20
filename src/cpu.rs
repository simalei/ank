use std::collections::HashMap;
use psutil::cpu::{cpu_count, CpuPercentCollector};
use anyhow::Result;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, LegendPosition};
use crate::cli::Args;
use crate::widget::Widget;

pub struct CPUWidget {
    title: String,
    cpu_percent_collector: CpuPercentCollector,
    avg_data: Vec<(f64, f64)>,
    percpu_data: HashMap<u64, Vec<(f64, f64)>>,
    update_count: f64
}


impl CPUWidget {
    pub fn new(title: &str) -> Self {

        let mut percpu_data: HashMap<u64, Vec<(f64, f64)>> = HashMap::new();
        for cpu in 0..cpu_count() {
            percpu_data.insert(cpu, vec![(0.0, 0.0)]);
        }

        Self {
            title: title.to_string(),
            cpu_percent_collector: CpuPercentCollector::new().unwrap(),
            avg_data: vec![(0.0, 0.0)],
            percpu_data,
            update_count: 0.0
        }
    }
}

impl Widget for CPUWidget {
    fn update(&mut self, args: &Args) -> Result<()> {
        self.update_count += 1.0;
        self.avg_data.push((self.update_count, self.cpu_percent_collector.cpu_percent()? as f64));


        if args.per_cpu {
            for cpu in 0..cpu_count() {
                let cpu_percent_vec = self.cpu_percent_collector.cpu_percent_percpu()?;
                let percpu_load = cpu_percent_vec.get(cpu as usize).unwrap();
                self.percpu_data.get_mut(&cpu).unwrap().push((self.update_count, *percpu_load as f64));
            }
        }


        Ok(())
    }

    fn draw(&self, f: &mut Frame, args: &Args, s: Rect) -> Result<()> {

        let mut datasets = vec![
            Dataset::default()
                .name(format!("CPU Avg ({:.1}%)", self.avg_data.last().unwrap().1))// TODO: Use structs instead of tuples
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::Indexed(1)))
                .graph_type(GraphType::Line)
                .data(&self.avg_data)
        ];
        
        if args.per_cpu {
            for x in 0..cpu_count() {

                let cpu_data = self.percpu_data.get(&x).unwrap();
                datasets.push(
                    Dataset::default()
                        .name(format!("CPU {} ({:.1}%)", x, cpu_data.last().unwrap().1))
                        .marker(Marker::Braille)
                        .style(Style::default().fg(Color::Indexed((x as u8) + 2))) // Zero is black, so we need to avoid that
                        .graph_type(GraphType::Line)
                        .data(cpu_data)
                );
            }
        }

        let chart = Chart::new(datasets)
            .legend_position(Some(LegendPosition::Left))
            .hidden_legend_constraints((Constraint::Min(0), Constraint::Min(0))) // Always show the legend. Maybe will be changed later
            .block(Block::new().title(self.title.to_owned()).borders(Borders::ALL))
            .x_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds([self.update_count - 25.0, self.update_count + 1.0])
            )
            .y_axis(
                Axis::default()
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 120.0]),
            );

        f.render_widget(chart, s);

        Ok(())
    }
}
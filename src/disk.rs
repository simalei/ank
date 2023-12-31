use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders, Gauge};
use crate::cli::Args;
use crate::widget::Widget;
use anyhow::Result;

use psutil::disk::*;
use ratatui::prelude::Direction;
use crate::{mb_or_gb, mb_or_gb_label};

struct Disk {
    pub partition: Partition,
    pub disk_usage: DiskUsage
}

impl Disk {
    fn new(partition: Partition) -> Self {
        let disk_usage = disk_usage(partition.mountpoint()).unwrap();
        Self {
            partition,
            disk_usage
        }
    }
}


pub struct DiskWidget {
    disks: Vec<Disk>,
    disk_count: u16,
    constraints: Vec<Constraint>
}

impl DiskWidget {


    pub fn new() -> Self {

        let mut disks: Vec<Disk> = vec![];
        for partition in partitions_physical().unwrap() {
            disks.push(Disk::new(partition));
        }

        let mut constraints: Vec<Constraint> = vec![];

        let disk_count = disks.len() as u16;

        let percentage = 100 / disk_count;

        for _ in 0..disk_count {
            constraints.push(Constraint::Percentage(percentage));
        }

        Self {
            disk_count,
            disks,
            constraints,
        }
    }
}

impl Widget for DiskWidget {
    fn update(&mut self, _args: &Args) -> Result<()> { // TODO: Figure out a way to update it, but not every second - that's too expensive
        let mut disks: Vec<Disk> = vec![];
        for partition in partitions_physical().unwrap() {
            disks.push(Disk::new(partition));
        }
        self.disk_count = disks.len() as u16;
        self.disks = disks;
        Ok(())
    }

    fn draw(&self, f: &mut Frame, args: &Args, s: Rect) -> Result<()> {

        let disk_layout = Layout::new(Direction::Vertical, self.constraints.clone()).split(s);

        //let block = Block::new().title("Disk Usage");

        let size_label = mb_or_gb_label!(args);

        for disk_num in 0..self.disk_count {
            let disk = &self.disks[disk_num as usize];


            let label = format!("{} {size_label}/{} {size_label} ({:.2}% used)",
                                mb_or_gb!(args, disk.disk_usage.used()),
                                mb_or_gb!(args, disk.disk_usage.total()),
                                disk.disk_usage.percent()
            );
            let gauge = Gauge::default()
                .label(label)
                .block(Block::new().title(format!(" {} • {} • {} ",
                                                  disk.partition.device(),
                                                  disk.partition.mountpoint().to_str().unwrap(),
                                                  disk.partition.filesystem().as_str()
                )).borders(Borders::ALL))
                .percent(disk.disk_usage.percent() as u16);
            f.render_widget(gauge, disk_layout[disk_num as usize]);
        }

        Ok(())
    }
}
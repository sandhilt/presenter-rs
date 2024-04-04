use entities::{elements::Table, presenter::{Interactor, View}};

use crate::entities::{elements::TableRow, presenter::Slide};

pub mod adapters;
pub mod entities;
pub mod use_cases;

fn main() {
    let my_slide = Slide::default();

    let mut elements: Vec<Box<dyn Interactor>> = vec![];
    let mut table = Table::default();

    let mut row = TableRow::default();
    row.add_cell("Hello".to_string());
    row.add_cell("World".to_string());
    row.add_cell("!!!".to_string());

    table.rows.push(row);

    let mut row = TableRow::default();
    row.add_cell("Dead".to_string());
    row.add_cell("beef".to_string());
    row.add_cell("!!!".to_string());

    table.rows.push(row);
    elements.push(Box::new(table));

    my_slide.render(elements);

    // println!("Hello, world!");
}

use super::presenter::{Interactor, Position, Size, Style};

#[derive(Default)]
pub struct Textbox {
    text: String,
    position: Position,
    size: Size,
    style: Style,
}

pub struct Image {
    position: Position,
    size: Size,
    source: String,
}

pub struct ImageBuilder {
    image: Image,
}

impl ImageBuilder {
    pub fn new(source: String) -> Self {
        Self {
            image: Image {
                position: Position::default(),
                size: Size::default(),
                source,
            },
        }
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.image.position = position;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.image.size = size;
        self
    }

    pub fn build(self) -> Image {
        self.image
    }
}

#[derive(Default)]
pub struct TableRow {
    cells: Vec<String>,
}

impl TableRow {
    pub fn add_cell(&mut self, cell: String) {
        self.cells.push(cell);
    }
}

impl Interactor for TableRow {
    fn get_data(&self) -> String {
        self.cells.join("\t|\t")
    }
}

#[derive(Default)]
pub struct Table {
    pub position: Position,
    pub size: Size,
    pub rows: Vec<TableRow>,
}

impl Interactor for Table {
    fn get_data(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.get_data())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

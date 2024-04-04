pub trait View {
    fn render(&self, data: Vec<Box<dyn Interactor>>);
}

pub trait Interactor {
    fn get_data(&self) -> String;
}

#[derive(Default)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Default)]
pub struct Size {
    width: f32,
    height: f32,
}

pub struct Color([u8; 4]);

pub struct Style {
    font_size: u8,
    font_color: Color,
    font_family: String,
    background_color: Color,
    border_color: Color,
    border_width: u8,
    border_radius: u8,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            font_size: 12,
            font_color: Color([0, 0, 0, 255]),
            font_family: "Arial".to_string(),
            background_color: Color([255, 255, 255, 255]),
            border_color: Color([0, 0, 0, 255]),
            border_width: 1,
            border_radius: 0,
        }
    }
}

pub struct Presenter {
    view: Box<dyn View>,
    interactor: Box<dyn Interactor>,
}

#[derive(Default)]
pub struct Slide {
    size: Size,
}

impl View for Slide {
    fn render(&self, data: Vec<Box<dyn Interactor>>) {
        for element in data {
            let data = element.get_data();
            println!("Rendering element: \n{}", data);
        }
    }
}

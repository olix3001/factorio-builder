use std::fmt::Debug;

use image::{GenericImageView, Rgb, RgbImage};
use imageproc::drawing::{
    draw_cross_mut, draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_circle_mut,
    draw_hollow_rect_mut, draw_line_segment_mut, draw_polygon_mut, draw_text_mut, Canvas,
};
use imageproc::point::Point;
use imageproc::rect::Rect;
use rusttype::Font;

static SIZE: u32 = 100;
static FONT_DATA: &[u8] = include_bytes!("../../consolas.ttf");

fn get_font<'a>() -> Font<'static> {
    Font::try_from_bytes(FONT_DATA).unwrap()
}

#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}
impl Vec2 {
    pub fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
}

#[derive(Debug)]
pub struct Blueprint {
    machines: Vec<Box<dyn Machine>>,
}

impl Blueprint {
    pub fn new() -> Blueprint {
        Blueprint { machines: vec![] }
    }

    pub fn add_machine(&mut self, machine: Box<dyn Machine>) {
        self.machines.push(machine);
    }

    pub fn add_machines(&mut self, machines: Vec<Box<dyn Machine>>) {
        self.machines.extend(machines);
    }

    pub fn remove_machine(&mut self, id: uuid::Uuid) {
        self.machines.retain(|m| m.id() != id);
    }

    pub fn machines(&self) -> &Vec<Box<dyn Machine>> {
        &self.machines
    }

    pub fn machines_mut(&mut self) -> &mut Vec<Box<dyn Machine>> {
        &mut self.machines
    }

    pub fn get_size(&self) -> Vec2 {
        let mut min = Vec2::new(i32::MAX, i32::MAX);
        let mut max = Vec2::new(i32::MIN, i32::MIN);

        for machine in self.machines() {
            if machine.position().x < min.x {
                min.x = machine.position().x;
            }
            if machine.position().y < min.y {
                min.y = machine.position().y;
            }
            if machine.position().x + machine.size().x > max.x {
                max.x = machine.position().x + machine.size().x;
            }
            if machine.position().y + machine.size().y > max.y {
                max.y = machine.position().y + machine.size().y;
            }
        }

        Vec2::new(max.x - min.x, max.y - min.y)
    }

    pub fn draw(&self) {
        // Create colors
        let black = Rgb([0, 0, 0]);
        let white = Rgb([255, 255, 255]);
        let light_gray = Rgb([200, 200, 200]);
        let dark_gray = Rgb([100, 100, 100]);

        // Create image
        let size = self.get_size();
        let mut img = RgbImage::from_pixel(size.x as u32 * SIZE, size.y as u32 * SIZE, white);

        // Draw background
        for x in 0..size.x {
            for y in 0..size.y {
                draw_hollow_rect_mut(
                    &mut img,
                    Rect::at(x * SIZE as i32, y * SIZE as i32).of_size(SIZE, SIZE),
                    light_gray,
                )
            }
        }

        // Draw machines
        for machine in self.machines() {
            machine.draw(&mut img);
        }

        // Save image
        img.save("test.png").unwrap();
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub trait Machine
where
    Self: Debug,
{
    fn id(&self) -> uuid::Uuid;
    fn name(&self) -> &str;
    fn position(&self) -> &Vec2;
    fn direction(&self) -> Option<&Direction>;
    fn size(&self) -> &Vec2;

    fn draw(&self, img: &mut RgbImage);
}

pub fn default_machine_draw(img: &mut RgbImage, machine: Box<dyn Machine>) {
    // Draw machine id in top left corner
    draw_text_mut(
        img,
        Rgb([0, 0, 0]),
        (machine.position().x as u32 * SIZE) as i32 + 2,
        (machine.position().y as u32 * SIZE) as i32 + 2,
        rusttype::Scale::uniform(12.0),
        &get_font(),
        machine.id().to_string().split("-").next().unwrap(),
    );

    // Draw machine name below id
    draw_text_mut(
        img,
        Rgb([0, 0, 0]),
        (machine.position().x as u32 * SIZE) as i32 + 2,
        (machine.position().y as u32 * SIZE) as i32 + 2 + 12,
        rusttype::Scale::uniform(12.0),
        &get_font(),
        machine.name(),
    );

    // Draw machine direction as a triangle
    let mut points = vec![];

    match machine.direction() {
        Some(Direction::South) => {
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24,
            ));
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2 + 12,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24,
            ));
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2 + 6,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24 + 12,
            ));
        }
        Some(Direction::East) => {
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24,
            ));
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24 + 12,
            ));
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2 + 12,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24 + 6,
            ));
        }
        Some(Direction::North) => {
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24 + 12,
            ));
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2 + 12,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24 + 12,
            ));
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2 + 6,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24,
            ));
        }
        Some(Direction::West) => {
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2 + 12,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24,
            ));
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2 + 12,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24 + 12,
            ));
            points.push(Point::new(
                (machine.position().x as u32 * SIZE) as i32 + 2,
                (machine.position().y as u32 * SIZE) as i32 + 2 + 24 + 6,
            ));
        }
        None => {}
    }

    draw_polygon_mut(img, &points, Rgb([150, 150, 150]));

    // Draw machine outline
    draw_hollow_rect_mut(
        img,
        Rect::at(
            (machine.position().x as u32 * SIZE) as i32,
            (machine.position().y as u32 * SIZE) as i32,
        )
        .of_size(
            (machine.size().x as u32 * SIZE) as u32,
            (machine.size().y as u32 * SIZE) as u32,
        ),
        Rgb([200, 0, 0]),
    );

    println!(
        "Drawing machine: {} ({})",
        machine.name(),
        machine.id().to_string()
    );
}

#[macro_export]
macro_rules! default_machine {
    ($machine:ty, $size:expr) => {
        impl $machine {
            pub fn new(pos: Vec2, dir: Option<Direction>) -> Self {
                Self {
                    id: uuid::Uuid::new_v4(),
                    position: pos,
                    size: $size,
                    direction: dir,
                }
            }
            pub fn new_boxed(pos: Vec2, dir: Option<Direction>) -> Box<Self> {
                Box::new(Self::new(pos, dir))
            }
        }
        impl Machine for $machine {
            fn id(&self) -> uuid::Uuid {
                self.id
            }

            fn name(&self) -> &str {
                stringify!($machine)
            }

            fn position(&self) -> &Vec2 {
                &self.position
            }

            fn direction(&self) -> Option<&Direction> {
                self.direction.as_ref()
            }

            fn size(&self) -> &Vec2 {
                &self.size
            }

            fn draw(&self, img: &mut image::RgbImage) {
                crate::factorio::components::default_machine_draw(img, Box::new(self.clone()));
            }
        }
    };
}

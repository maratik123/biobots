use crate::bot::Direction;
use crate::consts::{drawing, field};
use crate::{draw, Point, Rect, Size};
use egui::{Color32, ColorImage};
use std::array;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Images {
    pub apple: ColorImage,
    pub organics: ColorImage,
    pub rock: ColorImage,
    pub bot: BotImage,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct BotImage {
    pub head: [ColorImage; 8],
    pub body: ColorImage,
}

impl Default for Images {
    fn default() -> Self {
        Self {
            apple: create_apple(),
            organics: create_organics(),
            rock: create_rock(),
            bot: BotImage::default(),
        }
    }
}

fn create_apple() -> ColorImage {
    let mut apple = create_empty_square(field::CELL_SIZE);
    for y in 0..apple.height() {
        for x in 0..apple.width() {
            let (x_dist, y_dist) = (
                field::CELL_SIZE_HALF as isize - x as isize,
                field::CELL_SIZE_HALF as isize - y as isize,
            );
            if ((x_dist * x_dist) as usize + (y_dist * y_dist) as usize)
                < field::CELL_SIZE_HALF * field::CELL_SIZE_HALF
            {
                apple[(x, y)] = drawing::APPLE_DRAW_COLOR_RGBA;
            }
        }
    }
    apple
}

fn create_organics() -> ColorImage {
    let mut organics = create_empty_square(field::CELL_SIZE);
    draw::rect(
        &mut organics,
        &Rect {
            top_left: Point { x: 1, y: 1 },
            size: Size {
                w: field::CELL_SIZE - 2,
                h: field::CELL_SIZE - 2,
            },
        },
        drawing::ORGANIC_WASTE_OUTLINE_COLOR,
    );
    draw::filled_rect(
        &mut organics,
        &Rect {
            top_left: Point { x: 2, y: 2 },
            size: Size {
                w: field::CELL_SIZE - 4,
                h: field::CELL_SIZE - 4,
            },
        },
        drawing::ORGANIC_WASTE_DRAW_COLOR,
    );
    organics
}

fn create_rock() -> ColorImage {
    let mut rock = create_empty_square(field::CELL_SIZE);
    draw::filled_rect(
        &mut rock,
        &Rect {
            top_left: Point { x: 1, y: 1 },
            size: Size {
                w: field::CELL_SIZE - 2,
                h: field::CELL_SIZE - 2,
            },
        },
        drawing::ROCK_DRAW_COLOR,
    );
    rock
}

impl Default for BotImage {
    fn default() -> Self {
        let head = array::from_fn(|i| {
            let mut head = create_empty_square(field::CELL_SIZE);
            if let Some(color) = drawing::BOT_OUTLINE_COLOR {
                draw::rect(
                    &mut head,
                    &Rect {
                        top_left: Point { x: 0, y: 0 },
                        size: Size {
                            w: field::CELL_SIZE,
                            h: field::CELL_SIZE,
                        },
                    },
                    color,
                );
            }
            if let Some(color) = drawing::BOT_HEAD_COLOR {
                let half_size = Size {
                    w: field::CELL_SIZE_HALF,
                    h: field::CELL_SIZE_HALF,
                };
                let range_to_center = 0..field::CELL_SIZE_HALF;
                let range_from_center = field::CELL_SIZE_HALF..field::CELL_SIZE;
                let i: Direction = (i as u32).try_into().unwrap();
                match i {
                    Direction::N => draw::vertical_line(
                        &mut head,
                        field::CELL_SIZE_HALF,
                        range_to_center,
                        color,
                    ),
                    Direction::NW => draw::diagonal_line_from_top_left(
                        &mut head,
                        &Rect {
                            top_left: Point { x: 0, y: 0 },
                            size: half_size,
                        },
                        color,
                    ),
                    Direction::W => draw::horizontal_line(
                        &mut head,
                        range_to_center,
                        field::CELL_SIZE_HALF,
                        color,
                    ),
                    Direction::SW => draw::diagonal_line_from_bottom_left(
                        &mut head,
                        &Rect {
                            top_left: Point {
                                x: 0,
                                y: field::CELL_SIZE_HALF,
                            },
                            size: half_size,
                        },
                        color,
                    ),
                    Direction::S => draw::vertical_line(
                        &mut head,
                        field::CELL_SIZE_HALF,
                        range_from_center,
                        color,
                    ),
                    Direction::SE => draw::diagonal_line_from_top_left(
                        &mut head,
                        &Rect {
                            top_left: Point {
                                x: field::CELL_SIZE_HALF,
                                y: field::CELL_SIZE_HALF,
                            },
                            size: half_size,
                        },
                        color,
                    ),
                    Direction::E => draw::horizontal_line(
                        &mut head,
                        range_from_center,
                        field::CELL_SIZE_HALF,
                        color,
                    ),
                    Direction::NE => draw::diagonal_line_from_bottom_left(
                        &mut head,
                        &Rect {
                            top_left: Point {
                                x: field::CELL_SIZE_HALF,
                                y: 0,
                            },
                            size: half_size,
                        },
                        color,
                    ),
                }
            }
            head
        });
        let body = ColorImage::new([field::CELL_SIZE, field::CELL_SIZE], Color32::WHITE);
        Self { head, body }
    }
}

fn create_empty_square(size: usize) -> ColorImage {
    ColorImage::new([size, size], Color32::TRANSPARENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organics() {
        let organics = create_organics();
        assert_eq!(organics[(0, 0)], Color32::TRANSPARENT);
        assert_eq!(
            organics[(organics.width() - 1, organics.height() - 1)],
            Color32::TRANSPARENT
        );
        assert_eq!(organics[(1, 1)], drawing::ORGANIC_WASTE_OUTLINE_COLOR);
        assert_eq!(
            organics[(organics.width() - 2, organics.height() - 2)],
            drawing::ORGANIC_WASTE_OUTLINE_COLOR
        );
        assert_eq!(organics[(2, 2)], drawing::ORGANIC_WASTE_DRAW_COLOR);
        assert_eq!(
            organics[(organics.width() - 3, organics.height() - 3)],
            drawing::ORGANIC_WASTE_DRAW_COLOR
        );
    }
}

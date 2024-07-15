use crate::Rect;
use egui::{Color32, ColorImage};
use std::ops::Range;

pub fn filled_rect(image: &mut ColorImage, rect: &Rect<usize>, color: Color32) {
    if rect.is_empty() {
        return;
    }
    let bottom_right = rect.bottom_right();
    for y in rect.top_left.y..bottom_right.y {
        for x in rect.top_left.x..bottom_right.x {
            image[(x, y)] = color;
        }
    }
}

pub fn rect(image: &mut ColorImage, rect: &Rect<usize>, color: Color32) {
    if rect.is_empty() {
        return;
    }
    let bottom_right = rect.bottom_right();
    horizontal_line(
        image,
        rect.top_left.x..bottom_right.x,
        rect.top_left.y,
        color,
    );
    horizontal_line(
        image,
        rect.top_left.x..bottom_right.x,
        bottom_right.y - 1,
        color,
    );
    if rect.size.h > 2 {
        vertical_line(
            image,
            rect.top_left.x,
            rect.top_left.y + 1..bottom_right.y - 1,
            color,
        );
        vertical_line(
            image,
            bottom_right.x - 1,
            rect.top_left.y + 1..bottom_right.y - 1,
            color,
        );
    }
}

pub fn horizontal_line(image: &mut ColorImage, x: Range<usize>, y: usize, color: Color32) {
    for x in x {
        image[(x, y)] = color;
    }
}

pub fn vertical_line(image: &mut ColorImage, x: usize, y: Range<usize>, color: Color32) {
    for y in y {
        image[(x, y)] = color;
    }
}

pub fn diagonal_line_from_top_left(image: &mut ColorImage, rect: &Rect<usize>, color: Color32) {
    if rect.is_empty() {
        return;
    }
    let dx = rect.size.w as isize - 1;
    let dy = rect.size.h as isize - 1;
    let mut d = 2 * dy - dx;
    let mut y = rect.top_left.y;
    for x in rect.top_left.x..rect.bottom_right().x {
        image[(x, y)] = color;
        if d > 0 {
            y += 1;
            d -= 2 * dx;
        }
        d += 2 * dy;
    }
}

pub fn diagonal_line_from_bottom_left(image: &mut ColorImage, rect: &Rect<usize>, color: Color32) {
    if rect.is_empty() {
        return;
    }
    let dx = rect.size.w as isize - 1;
    let dy = rect.size.h as isize - 1;
    let mut d = 2 * dy - dx;
    let bottom_right = rect.bottom_right();
    let mut y = bottom_right.y;
    for x in rect.top_left.x..bottom_right.x {
        image[(x, y - 1)] = color;
        if d > 0 {
            y -= 1;
            d -= 2 * dx;
        }
        d += 2 * dy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Size};

    #[test]
    fn test_diagonal_line_from_top_left() {
        let mut image = ColorImage::new([8, 8], Color32::PLACEHOLDER);
        diagonal_line_from_top_left(
            &mut image,
            &Rect {
                top_left: Point { x: 1, y: 1 },
                size: Size { w: 6, h: 6 },
            },
            Color32::DEBUG_COLOR,
        );
        for x in 0..8 {
            for y in 0..8 {
                assert_eq!(
                    image[(x, y)],
                    if x != 0 && x != 7 && x == y {
                        Color32::DEBUG_COLOR
                    } else {
                        Color32::PLACEHOLDER
                    }
                );
            }
        }
    }

    #[test]
    fn test_diagonal_line_from_bottom_left() {
        let mut image = ColorImage::new([8, 8], Color32::PLACEHOLDER);
        diagonal_line_from_bottom_left(
            &mut image,
            &Rect {
                top_left: Point { x: 1, y: 1 },
                size: Size { w: 6, h: 6 },
            },
            Color32::DEBUG_COLOR,
        );
        for x in 0..8 {
            for y in 0..8 {
                assert_eq!(
                    image[(x, y)],
                    if x != 0 && x != 7 && x == 7 - y {
                        Color32::DEBUG_COLOR
                    } else {
                        Color32::PLACEHOLDER
                    },
                    "at {x}, {y}"
                );
            }
        }
    }
}

use std::ops::{Add, AddAssign};

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point<T: Copy> {
    pub x: T,
    pub y: T,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Size<T: Copy> {
    pub w: T,
    pub h: T,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Rect<T: Copy> {
    pub top_left: Point<T>,
    pub size: Size<T>,
}

impl<T> AddAssign for Point<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> AddAssign<Size<T>> for Point<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Size<T>) {
        self.x += rhs.w;
        self.y += rhs.h;
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Add<Size<T>> for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Size<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.w,
            y: self.y + rhs.h,
        }
    }
}

impl<T> Size<T>
where
    T: Default + Copy + PartialEq,
{
    pub fn is_empty(&self) -> bool {
        let zero = T::default();
        self.w == zero || self.h == zero
    }
}

impl<T> Rect<T>
where
    T: Default + Copy + PartialEq,
{
    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Copy,
{
    pub fn bottom_right(&self) -> Point<T> {
        self.top_left + self.size
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + PartialOrd<T> + Copy,
{
    pub fn is_in_bounds(&self, point: Point<T>) -> bool {
        point.x >= self.top_left.x && point.y >= self.top_left.y && {
            let bottom_right = self.bottom_right();
            point.x < bottom_right.x && point.y < bottom_right.y
        }
    }
}

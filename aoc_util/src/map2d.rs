use std::alloc;
use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut};
use std::str::Lines;
use std::{alloc::Layout, ptr::NonNull};

use crate::vec2::Vec2;

pub struct Map2D<T> {
    width: usize,
    height: usize,
    map: NonNull<T>
}

impl<T> Map2D<T> {

    fn new(width: usize, height: usize) -> Self {
        let cap = width * height;

        let arr_layout = Layout::array::<T>(cap).unwrap();
        let alloced: *mut T = unsafe { alloc::alloc(arr_layout) } as *mut T;

        Self {
            width,
            height,
            map: match NonNull::new(alloced) {
                Some(p) => p,
                None => alloc::handle_alloc_error(arr_layout)
            },
        }
    }

    pub fn from_chars<MapFn>(lines: Lines, mut mapfn: MapFn) -> Option<Self> 
        where MapFn: FnMut(char, (usize, usize)) -> T
    {

        let mut peekable = lines.clone().peekable();

        let width: usize = match peekable.peek() {
            Some(l) => l.chars().count(),
            None => return None,
        };
        let height = peekable.count();

        if width * height == 0 {
            return None;
        }

        let mut out = Self::new(width, height);

        for (r, line) in lines.enumerate() {
            for (c, char) in line.chars().enumerate() {
                out.set(r,c, mapfn(char, (r,c)));
            }
        }

        return Some(out);
    }

    // No checks because I am the only one using this and idgaf
    pub fn set(&mut self, r: usize, c: usize, v: T) {
        unsafe { 
            self.map.offset((r * self.width + c) as isize)
                .write(v)
        };
    }

    pub fn get(&self, r: usize, c: usize) -> T {
        unsafe {
            self.map.offset((r * self.width + c) as isize)
                .read()
        }
    }

    pub fn set_v(&mut self, pos: Vec2<usize>, v: T) {
        self.set(pos.y, pos.x, v);
    }

    pub fn get_v(&self, pos: Vec2<usize>) -> T {
        self.get(pos.y, pos.x)
    }

    pub fn in_bounds(&self, r: isize, c: isize) -> bool {
        return
            r >= 0 &&
            r < self.height as isize &&
            c >= 0 &&
            c < self.width as isize;
    }

    pub fn in_bounds_v(&self, pos: Vec2<usize>) -> bool {
        return self.in_bounds(pos.y as isize, pos.x as isize);
    }

    pub fn in_bounds_iv(&self, pos: Vec2<isize>) -> bool {
        return self.in_bounds(pos.y, pos.x);
    }

    // Directions are those of unit circle: 0 = right, 1 = up, 2 = left, 3 = down
    pub fn bounded_move_in_dir(&self, pos: Vec2<usize>, dir: usize) -> Option<Vec2<usize>> {
        let d = dir % 4;
        match d {
            0 => if pos.x < self.width - 1 {
                Some(Vec2{ x: pos.x+1, y: pos.y })
            } else {
                None
            }
            1 => if pos.y > 0 {
                Some(Vec2{ x: pos.x, y: pos.y-1 })
            } else {
                None
            }
            2 => if pos.x > 0 {
                Some(Vec2{ x: pos.x-1, y: pos.y })
            } else {
                None
            }
            // i.e. 3, given mod
            _ => if pos.y < self.height - 1 {
                Some(Vec2{ x: pos.x, y: pos.y+1 })
            } else {
                None
            }
        }
    }
}

impl<T> Display for Map2D<T> 
    where T: Display {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for r in 0..self.height {
            for c in 0..self.width {
                if let Err(e) = write!(f, "{}", self.get(r,c)) {
                    return Err(e);
                }
            }
            if let Err(e) = writeln!(f, "") {
                return Err(e);
            }
        }

        return Ok(());
    }
}

impl<T> Debug for Map2D<T> 
    where T: Debug {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for r in 0..self.height {
            for c in 0..self.width {
                if let Err(e) = write!(f, "{:?}", self.get(r,c)) {
                    return Err(e);
                }
            }
            if let Err(e) = writeln!(f, "") {
                return Err(e);
            }
        }

        return Ok(());
    }
}

impl<T> Index<usize> for Map2D<T> {

    type Output = [T];

    fn index(&self, r: usize) -> &Self::Output {
        return unsafe {
            std::slice::from_raw_parts(
                self.map.offset((r * self.width) as isize).as_ptr(),
                self.width)
        }
    }
}

impl<T> IndexMut<usize> for Map2D<T> {

    fn index_mut(&mut self, r: usize) -> &mut [T] {
        return unsafe {
            std::slice::from_raw_parts_mut(
                self.map.offset((r * self.width) as isize).as_ptr(),
                self.width)
        }
    }
}

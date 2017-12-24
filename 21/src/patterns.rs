extern crate std;

#[derive(Debug, Hash, Eq, Clone)]
pub struct Pattern {
    size: usize,
    pixels: Vec<bool>,
}

impl Pattern {
    /// Returns new empty `Pattern` with given size.
    pub fn new(size: usize) -> Pattern {
        Pattern {
            size,
            pixels: vec![false; size * size],
        }
    }

    /// Returns new pattern with given size and `Vec` of pixels.
    pub fn from_vec(size: usize, pixels: Vec<bool>) -> Pattern {
        Pattern {
            size,
            pixels,
        }
    }

    /// Returns new pattern parsed from given `str`.
    pub fn from_str(s: &str) -> Pattern {
        let mut pixels = vec![];
        let mut size = 0;
        for row in s.split('/') {
            for pixel in row.chars() {
                pixels.push(pixel == '#');
            }
            size += 1;
        }
        Pattern::from_vec(size, pixels)
    }

    /// Returns size of the patterns. Since patterns are squares,
    /// the returned number is equal to amount of rows and columns.
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// Returns one big pattern produced from a slice of smaller patterns with the same size.
    ///
    /// Note: length of the slice *must* be some square of some number.
    ///
    /// Example for slice of 9 patterns:
    ///  [0][1][2]
    ///  [3][4][5]
    ///  [6][7][8]
    ///
    pub fn merge(patterns: &[Pattern]) -> Pattern {
        let size = Self::infer_size_from_count(patterns.len());
        let subpattern_size = patterns
            .first()
            .expect("some patterns to merge")
            .get_size();

        let mut merged = Pattern::new(subpattern_size * size);
        for (idx, pattern) in patterns.iter().enumerate() {
            let offset_row = (idx / size) * subpattern_size;
            let offset_column = (idx % size) * subpattern_size;
            merged.put_subpattern(pattern, offset_row, offset_column);
        }
        merged
    }

    fn infer_size_from_count(count: usize) -> usize {
        (count as f64).sqrt() as usize
    }

    fn put_subpattern(&mut self, subpattern: &Pattern, offset_row: usize, offset_column: usize) {
        for row in 0..subpattern.size {
            for column in 0..subpattern.size {
                self.set_pixel(
                    offset_row + row,
                    offset_column + column,
                    subpattern.get_pixel(row, column),
                );
            }
        }
    }

    /// Returns `Vec` of subpatterns created from the `Pattern`.
    /// When pattern has size divisible by 2
    pub fn split(&self) -> Vec<Pattern> {
        let size = if self.size % 2 == 0 {
            2
        } else if self.size % 3 == 0 {
            3
        } else {
            unimplemented!()
        };
        let mut subpatterns = vec![];
        for offset_row in (0..self.size).step_by(size) {
            for offset_column in (0..self.size).step_by(size) {
                subpatterns.push(
                    self.extract_subpattern(size, offset_row, offset_column)
                );
            }
        }
        subpatterns
    }

    fn extract_subpattern(&self, size: usize, offset_row: usize, offset_column: usize) -> Pattern {
        let mut subpattern = Pattern::new(size);
        for row in 0..size {
            for column in 0..size {
                let pixel = self.get_pixel(offset_row + row, offset_column + column);
                subpattern.set_pixel(row, column, pixel);
            }
        }
        subpattern
    }

    #[inline]
    fn set_pixel(&mut self, row: usize, column: usize, value: bool) {
        assert!(row < self.size);
        assert!(column < self.size);
        self.pixels[row * self.size + column] = value;
    }

    #[inline]
    fn get_pixel(&self, row: usize, column: usize) -> bool {
        assert!(row < self.size);
        assert!(column < self.size);
        self.pixels[row * self.size + column]
    }

    /// Returns number of set pixels on the pattern.
    pub fn count_set_pixels(&self) -> usize {
        self.pixels.iter().filter(|&pixel| *pixel).count()
    }

    /// Checks whether one pattern was created by applying some rotation and/or flip to the other.
    pub fn is_variation_of(&self, other: &Pattern) -> bool {
        if self.size != other.size {
            return false;
        }

        match self.size {
            2 => self.is_variation_of_2x2(&other),
            3 => self.is_variation_of_3x3(&other),
            _ => unimplemented!()
        }
    }

    fn is_variation_of_2x2(&self, other: &Pattern) -> bool {
        let s1 = self.get_2x2_sequence();
        let s2 = other.get_2x2_sequence();

        is_rotation_of(&s1, &s2, 1) || is_rotation_of(&s2, &s1, 1)
    }

    fn is_variation_of_3x3(&self, other: &Pattern) -> bool {
        if self.get_pixel(1, 1) != other.get_pixel(1, 1) {
            return false;
        }
        let s1 = self.get_3x3_sequence();
        let mut s2 = other.get_3x3_sequence();
        if is_rotation_of(&s1, &s2, 2) || is_rotation_of(&s2, &s1, 2) {
            return true;
        }
        s2.reverse();
        s2.rotate(1);
        if is_rotation_of(&s1, &s2, 2) || is_rotation_of(&s2, &s1, 2) {
            return true;
        }
        false
    }

    fn get_2x2_sequence(&self) -> [bool; 4] {
        [
            self.get_pixel(0, 0),
            self.get_pixel(0, 1),
            self.get_pixel(1, 1),
            self.get_pixel(1, 0),
        ]
    }

    fn get_3x3_sequence(&self) -> [bool; 8] {
        [
            self.get_pixel(0, 0),
            self.get_pixel(0, 1),
            self.get_pixel(0, 2),
            self.get_pixel(1, 2),
            self.get_pixel(2, 2),
            self.get_pixel(2, 1),
            self.get_pixel(2, 0),
            self.get_pixel(1, 0),
        ]
    }
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Write;

        for (idx, &pixel) in self.pixels.iter().enumerate() {
            let row = idx / self.size;
            let column = idx % self.size;
            let is_last_row = row == self.size - 1;
            let is_last_column = column == self.size - 1;

            let _ = f.write_char(if pixel { '#' } else { '.' });
            if is_last_column && !is_last_row {
                let _ = f.write_char('\n');
            }
        }
        Ok(())
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.pixels == other.pixels
    }
}

fn is_rotation_of<T: PartialEq>(first: &[T], second: &[T], step: usize) -> bool {
    if first.len() != second.len() {
        return false;
    }
    let n = first.len();
    for i in (0..n).step_by(step) {
        if (0..n).all(|j| first[j] == second[(j + i) % n]) {
            return true;
        }
    }
    false
}
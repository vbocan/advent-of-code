use regex::Regex;

static INPUT: &'static str = include_str!("../input.txt");

struct Grid1 {
    n_rows: usize,
    n_cols: usize,
    data: Vec<bool>,
}

impl Grid1 {
    fn new(rows: usize, cols: usize) -> Grid1 {
        Grid1 {
            n_rows: rows,
            n_cols: cols,
            data: vec![false; rows * cols],
        }
    }
    fn set(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for j in y1..(y2 + 1) {
            for i in x1..(x2 + 1) {
                self.data[i + j * self.n_cols] = true;
            }
        }
    }
    fn unset(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for j in y1..(y2 + 1) {
            for i in x1..(x2 + 1) {
                self.data[i + j * self.n_cols] = false;
            }
        }
    }
    fn toggle(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for j in y1..(y2 + 1) {
            for i in x1..(x2 + 1) {
                self.data[i + j * self.n_cols] = !self.data[i + j * self.n_cols];
            }
        }
    }
    fn light_count(&mut self) -> u32 {
        self.data.iter().filter(|&x| *x == true).count() as u32
    }
}

struct Grid2 {
    n_rows: usize,
    n_cols: usize,
    data: Vec<u32>,
}

impl Grid2 {
    fn new(rows: usize, cols: usize) -> Grid2 {
        Grid2 {
            n_rows: rows,
            n_cols: cols,
            data: vec![0; rows * cols],
        }
    }
    fn set(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for j in y1..(y2 + 1) {
            for i in x1..(x2 + 1) {
                self.data[i + j * self.n_cols] = self.data[i + j * self.n_cols] + 1;
            }
        }
    }
    fn unset(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for j in y1..(y2 + 1) {
            for i in x1..(x2 + 1) {
                if self.data[i + j * self.n_cols] > 0 {
                    self.data[i + j * self.n_cols] = self.data[i + j * self.n_cols] - 1;
                }
            }
        }
    }
    fn toggle(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for j in y1..(y2 + 1) {
            for i in x1..(x2 + 1) {
                self.data[i + j * self.n_cols] = self.data[i + j * self.n_cols] + 2;
            }
        }
    }
    fn light_count(&mut self) -> u32 {
        let sum: u32 = self.data.iter().sum();
        sum
    }
}

fn main() {
    let mut grid1 = Grid1::new(1000, 1000);
    let mut grid2 = Grid2::new(1000, 1000);

    let re = Regex::new(r"([a-zA-Z\s]*)([0-9]*),([0-9]*)([a-zA-Z\s]*)([0-9]*),([0-9]*)").unwrap();

    for line in INPUT.lines() {
        println!("Line: {}", line);
        for cap in re.captures_iter(line) {
            let command = cap[1].trim();
            let x1 = *&cap[2].parse::<u32>().unwrap() as usize;
            let y1 = *&cap[3].parse::<u32>().unwrap() as usize;
            let x2 = *&cap[5].parse::<u32>().unwrap() as usize;
            let y2 = *&cap[6].parse::<u32>().unwrap() as usize;

            println!(
                "Command: {:?}, X1: {:?}, Y1: {:?}, X2: {:?}, Y2: {:?}",
                &command, &x1, &y1, &x2, &y2
            );

            match command {
                "turn on" => {
                    grid1.set(x1, y1, x2, y2);
                    grid2.set(x1, y1, x2, y2);
                }
                "turn off" => {
                    grid1.unset(x1, y1, x2, y2);
                    grid2.unset(x1, y1, x2, y2);
                }
                "toggle" => {
                    grid1.toggle(x1, y1, x2, y2);
                    grid2.toggle(x1, y1, x2, y2);
                }
                _ => {}
            }            
        }
    }
    println!("Grid 1: {} lights", grid1.light_count());
    println!("Total brightness for grid 2: {}", grid2.light_count());
}

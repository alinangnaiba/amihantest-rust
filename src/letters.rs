trait Display {
    fn draw(&self);
}

struct O {
    size : u32
}

struct X {
    size: u32
}

struct Y {
    size: u32
}

struct Z {
    size: u32
}

impl Display for O {
    fn draw(&self) {
        let len = self.size - 1;
        for row in 0..self.size {
            self.print_letter(len, row);
            println!("");
        }
    }
}

impl O {
    fn print_letter(&self, len: u32, row: u32) {
        for col in 0..self.size {
            if row == 0 || row == len {
                self.print_row(len, col);
            } else {
                self.print_col(len, col);
            }
        }
    }

    fn print_row(&self, len: u32, col: u32) {
        if col > 0 && col < len {
            print!("*");
        } else {
            print!(" ");
        } 
    }

    fn print_col(&self, len: u32, col: u32) {
        if col == 0 || col == len {
            print!("*");
        } else {
            print!(" ");
        }
    }
}

impl Display for X {
    fn draw(&self) {
        let len = self.size - 1;
        for row in 0..self.size {
            self.print_letter(len, row);
            println!("");
        }
    }
}

impl X {
    fn print_letter(&self, len: u32, row: u32) {
        for col in 0..self.size {
            if col == row || col == len - row {
                print!("*");
            } else {
                print!(" ");
            }
        }
    }
}


impl Display for Y {
    fn draw(&self) {
        let mid = self.size / 2;
        let len = self.size - 1;
        for row in 0..self.size {
            self.print_letter(len, row, mid);
            println!("");
        }
    }
}

impl Y {
    fn print_letter(&self, len: u32, row: u32, mid: u32) {
        for col in 0..self.size {
            if row <= mid 
            && (col == row || col == len - row)
            || (row > mid && col == mid) {
                print!("*");
            } else {
                print!(" ");
            }
        }
    }
}


impl Display for Z {
    fn draw(&self) {
        let len: u32 = self.size - 1;
        for row in 0..self.size {
            self.print_letter(len, row);
            println!("");
        }
    }
}

impl Z {
    fn print_letter(&self, len: u32, row: u32) {
        for col in 0..len {
            if row == 0 || row == len || col == len - row {
                print!("*");
                continue;
            } else {
                print!(" ");
            }
        }
    }
}

pub fn draw_letter(choice: &String, size: u32) {
    let letter = choice.trim();
    match letter {
        "O" =>  {
            let o: O = O {size: size};
            Display::draw(&o);
        },
        "X" => {
            let x: X = X { size: size };
            Display::draw(&x);
        },
        "Y" => {
            let y: Y = Y { size: size };
            Display::draw(&y);
        },
        _ => {
            let z: Z = Z { size: size };
            Display::draw(&z);
        }
    }
}



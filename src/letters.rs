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
            for col in 0..self.size {
                if row == 0 || row == len {
                    if col > 0 && col < len {
                        print!("*");
                    } else {
                        print!(" ");
                    } 
                } else {
                    if col == 0 || col == len {
                        print!("*");
                    } else {
                        print!(" ");
                    }
                }
            }
            println!("");
        }
    }
}

impl Display for Z {
    fn draw(&self) {
        let len: u32 = self.size - 1;
        for row in 0..self.size {
            for col in 0..len {
                if row == 0 || row == len || col == len - row {
                    print!("*");
                    continue;
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}

impl Display for X {
    fn draw(&self) {
        let len = self.size - 1;
        for row in 0..self.size {
            for col in 0..self.size {
                if col == row || col == len - row {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}

impl Display for Y {
    fn draw(&self) {
        let mid = self.size / 2;
        let len = self.size - 1;
        for row in 0..self.size {
            for col in 0..self.size {
                if row <= mid {
                    if col == row || col == len - row {
                        print!("*");
                    }
                }
                if row > mid && col == mid {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!("");
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



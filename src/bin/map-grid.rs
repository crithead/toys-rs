//
// An example game grid struct.
//

//struct Tile {
//    value: u32;
//}

struct Grid {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl Grid {
    // Create a new Grid with the specified width and height
    fn new(w: usize, h: usize) -> Grid {
        let length = w * h;
        let mut g = Grid {
            width: w,
            height: h,
            data: Vec::new(),
        };
        // Vec::new() creates an empty Vec,
        // resize sets the size to 'length' and inits the new elements to 0
        g.data.resize(length, 0);
        g
    }

    // Print information about this Grid
    fn print_info(&self) {
        println!("width\t{}", self.width);
        println!("height\t{}", self.height);
        println!("data length\t{}", self.data.len());
    }

    // Set all values to zero
    fn clear(&mut self) {
        for i in &mut self.data {
            *i = 0;
        }
    }

    // Get the value at (x, y)
    fn get(&self, x: usize, y: usize) -> u32 {
        let i = self.to_index(x, y);
        self.data[i]
    }

    // Set the value at (x, y) to v
    fn set(&mut self, x: usize, y: usize, v: u32) {
        let i = self.to_index(x, y);
        self.data[i] = v;
    }

    // Convert (x, y) coordinates to a Vec index.
    fn to_index(&self, x: usize, y: usize) -> usize {
        if x >= self.width {
            panic!("X out of range: x {} >= width {}", x, self.width)
        }
        if y >= self.height {
            panic!("Y out of range: y {} >= height {}", y, self.height)
        }
        y * self.width + x
    }
}

fn main() {
    println!("--- Grid #1 -- Integers");
    let mut g = Grid::new(3, 3);
    g.print_info();

    println!("new grid");
    print(&g);

    let mut v = 100;
    for r in 0..3 {
        for c in 0..3 {
            g.set(c, r, v);
            v += 10;
        }
    }

    println!("altered grid");
    print(&g);

    g.clear();
    println!("cleared grid");
    print(&g);

    println!("--- Grid #2 -- Tiles");
    println!("--- Grid #3 -- Generic");
}

fn print(g: &Grid) {
    for y in 0..g.height {
        for x in 0..g.width {
            print!("\t{}", g.get(x, y));
        }
        println!("");
    }
}

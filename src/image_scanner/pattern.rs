pub struct Pattern {
    coordinates: Vec<Coordinate>,
}

impl Pattern {
    pub fn new() -> Self {
        Pattern {
            coordinates: Vec::new(),
        }
    }
    pub fn add_coordinate(&mut self, x: usize, y: usize) {
        let coordinate = Coordinate { x, y };
        self.coordinates.push(coordinate);
    }

    pub fn get_coordinates(&self) -> &Vec<Coordinate> {
        &self.coordinates
    }
}

pub struct Coordinate {
    x: usize,
    y: usize,
}

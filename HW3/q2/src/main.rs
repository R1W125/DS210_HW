use std::f64::consts::PI;


fn main() {
    let sides_list: [u32;10] = [4, 8, 16, 32, 64, 128, 256, 512, 2048, 65536];
    let length_list: [f64;3] = [2.0,5.0,10.0];
    for side in &sides_list {
        for length in &length_list {
            let polygon = Polygon{sides: *side, lengths: *length};

            let poly_per: f64 = polygon.perimeter();
            let poly_area: f64 = polygon.area();
            let poly_rad: f64 = polygon.radius();
            let poly_apo: f64 = polygon.apothem();
            let cir_area: f64 = PI * poly_rad.powi(2);

            println!("{:?} \n perimeter: {}\n area:{}\n, radius: {}\n, apothem: {}\n, circle area: {}",
                        polygon, poly_per, poly_area, poly_rad, poly_apo, cir_area);
            println!("_____________________________");
        }
    }
}

trait Functions {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn radius(&self) -> f64;
    fn apothem(&self) -> f64;
}

#[derive(Debug)]
struct Polygon {
    sides: u32,
    lengths: f64,
}

//run an implementation

impl Functions for Polygon {
    fn area(&self) -> f64{
        (self.sides as f64 * self.lengths.powi(2)) / (4.0 * (PI / self.sides as f64).tan())
    }
    fn perimeter(&self) -> f64{
        self.sides as f64 * self.lengths
    }
    fn radius(&self) -> f64{
        self.lengths / (2.0 * (PI / self.sides as f64).sin())
    }
    fn apothem(&self) -> f64{
        self.lengths / (2.0 * (PI / self.sides as f64).tan())
    }
    
}
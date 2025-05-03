use std::f64::consts::PI;


fn main() {
    let mut pyramid1 = Shape::create("Pyramid",2.0,Some(2.0),Some(3.0));
    println!("pyramid1: {:?}", pyramid1);
    println!("pyramid1 volume: {}",pyramid1.volume());
    println!("pyramid1 surface area: {}",pyramid1.surface_area());
    pyramid1.double();
    println!("pyramid1: {:?}", pyramid1);
    println!("pyramid1 has the correct parameters: {}", pyramid1.verify());

    println!();

    let mut cuboid1 = Shape::create("Cuboid",2.0,Some(2.0),Some(3.0));
    println!("cuboid1: {:?}", cuboid1);
    println!("cuboid1 volume: {}",cuboid1.volume());
    println!("cuboid1 surface area: {}",cuboid1.surface_area());
    cuboid1.double();
    println!("cuboid1: {:?}", cuboid1);
    println!("cuboid1 has the correct parameters: {}", cuboid1.verify());

    println!();

    let mut sphere1 = Shape::create("Sphere",2.0,None,None);
    println!("sphere1: {:?}", sphere1);
    println!("sphere1 volume: {}",sphere1.volume());
    println!("sphere1 surface area: {}",sphere1.surface_area());
    sphere1.double();
    println!("sphere1: {:?}", sphere1);
    println!("sphere1 has the correct parameters: {}", sphere1.verify());

    println!();
    println!("Tests: \n");

    let pyramid2: Shape = Shape::create("Pyramid", -2.0, Some(-2.0), Some(4.6));
    println!("pyramid2: {:?}", pyramid2);
    println!("pyramid2 has the correct parameters: {}", pyramid2.verify());

    println!();

    let cuboid2 = Shape::create("Cuboid", -1.0,Some(2.0), Some(4.7));
    println!("cuboid2: {:?}", cuboid2);
    println!("cuboid2 has the correct parameters: {}", cuboid2.verify());

    println!();

    let sphere2 = Shape::create("Sphere",-2.0,None,None);
    println!("sphere2: {:?}", sphere2);
    println!("sphere2 has the correct parameters: {}", sphere2.verify());


}

#[derive(Debug)]
enum Shape{
    Pyramid(f64,f64,f64),
    Cuboid(f64,f64,f64),
    Sphere(f64),
}
impl Shape{
    fn create(shape:&str, l:f64, w:Option<f64>, h:Option<f64>) -> Self{
        match shape {
            "Pyramid" => Self::Pyramid(l,w.unwrap(),h.unwrap()),
            "Cuboid" => Self::Cuboid(l,w.unwrap(),h.unwrap()),
            "Sphere" => Self::Sphere(l),
            _ => todo!("Invalid shape type: {}",shape),
        }
    }
    fn volume(&self) -> f64{
        match self {
            Shape::Pyramid(l,w,h) => (*l * *w * *h)/3.0,
            Shape::Cuboid(l,w,h) => *l * *w * *h,
            Shape::Sphere(r) => (4.0/3.0) * PI * r.powi(2),
        }
    }
    fn surface_area(&self) -> f64{
        match self {
            Shape::Pyramid(l,w,h) => (*l * *w) + l * ((*w/2.0).powi(2)+ h.powf(2.0)).sqrt() + *w * ((*l/2.0).powi(2)+ h.powf(2.0)).sqrt(),
            Shape::Cuboid(l,w,h) => (2.0 * *l * *w)+(2.0 * *w * *h)+(2.0 * *l * *h),
            Shape::Sphere(r) => 4.0*PI*r.powi(2), 
        }
    }
    fn double(&mut self){
        match self {
            Shape::Pyramid(l,w,h) => {
                *l *= 2.0;
                *w *= 2.0;
                *h *= 2.0;
            },
            Shape::Cuboid(l,w,h) => {
                *l *= 2.0;
                *w *= 2.0;
                *h *= 2.0;
            },
            Shape::Sphere(r) => {
                *r *= 2.0;
            },
        }
    }
    fn verify(&self) -> bool{
        match self {
            Shape::Pyramid(l,w,h) => *l >0.0 && *w >0.00 && *h >0.00,
            Shape::Cuboid(l,w,h) => *l >0.0 && *w >0.00 && *h >0.00,
            Shape::Sphere(r) => *r >0.0,
        }
    }
}



use std::ops::Neg;

fn main() {
    let p1:Point<f32> = Point{x: 1.32, y: 3.33};
    let p2:Point<i64> = Point{x: 1976, y: 223};
    let p1_clock = p1.clockwise();
    let p2_counter = p2.counterclockwise();
    println!("Original P1: {:?} \nClockwise P1: {:?}",p1, p1_clock);
    println!();
    println!("Original P2: {:?} \nCounterclockwise P2: {:?}", p2, p2_counter);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point<T> {
    x:T,
    y:T,
}

impl <T:Copy + Neg<Output = T>> Point<T>{
    fn clockwise(&self) -> Point<T>{
        Point {
            x: self.y,
            y: -self.x,
        }
        
    }
    fn counterclockwise(&self) -> Point<T>{
        Point { x: -self.y, y: self.x }
    }
}


#[test]
fn clockwise_works(){
    let clock: Point<f64> = Point{x:1.0, y:3.0};
    let result = clock.clockwise();
    let expected: Point<f64> = Point{x:3.0,y:-1.0};
    assert_eq!(result, expected, "Fail!!!");
}

#[test]
fn counterclockwise_works(){
    let clock: Point<f64> = Point{x:1.0, y:3.0};
    let result = clock.counterclockwise();
    let expected: Point<f64> = Point{x:-3.0,y:1.0};
    assert_eq!(result, expected, "Fail!!!");
}


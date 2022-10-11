struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointXY<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointXY<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointXY<X2, Y2>) -> PointXY<X1, Y2> {
        PointXY {
            x: self.x,
            y: other.y,
        }
    }
}

struct PointTU<T, U> {
    x: T,
    y: U,
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'c', 'a'];
    let result = largest(&char_list);
    println!("The largest character is {}", result);

    let _both_integer = Point { x: 5, y: 10 };
    let _both_float = Point { x: 1.0, y: 4.0 };
    let _integer_and_float = PointTU { x: 5, y: 4.0 };
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    let pxy1 = PointXY { x: 5, y: 10.4 };
    let pxy2 = PointXY { x: "hello", y: 'c' };
    let pxy3 = pxy1.mixup(pxy2);
    println!("pxy3.x = {}, pxy3.y = {}", pxy3.x, pxy3.y);
}

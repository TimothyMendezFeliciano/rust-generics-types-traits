struct Point<T> {
    x: T,
    y: T,
}

struct BothPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<f32> Point<f32> {
    fn distance_from_original(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}



fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);

    println!("The largest number is {result}");

    let integer = Point { x: 5, y: 10 }; // Must be both of the same time.
    let float = Point { x: 1.0, y: 5.0 }; // Can be of different type. Only x, and y need to match type.

    let integer_and_float = BothPoint { x: 1, y: 4.0 };

    println!("integer.x = {}", integer.x());
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in &list {
        if number > largest {
            largest = number;
        }
    }

    return largest;
}

mod Traits;

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

impl<F32> Point<F32> {
    fn distance_from_original(&self) -> F32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixupPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> MixupPoint<X1, Y1> {
    fn mixup<X2, Y2>(
        self,
        other: MixupPoint<X2, Y2>,
    ) -> MixupPoint<X1, Y2> {
        MixupPoint {
            x: self.x,
            y: other.y,
        }
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

    let p1 = MixupPoint { x: 5, y: 10.4 };
    let p2 = MixupPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    return largest;
}

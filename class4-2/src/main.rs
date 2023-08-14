use std::ops::Add;

#[derive(Debug)]
struct MyNumber(i32);

impl Add for MyNumber {
    type Output = MyNumber;

    fn add(self, other: MyNumber) -> MyNumber {
        MyNumber(self.0 + other.0)
    }
}

trait Addable {
    fn add_self(&self) -> MyNumber;
}

impl Addable for MyNumber {
    fn add_self(&self) -> MyNumber {
        MyNumber(self.0 + self.0)
    }
}

fn call_addable<T>(addable: T)
where
    T: Addable,
{
    let result = addable.add_self();
    println!("Result of add_self: {:?}", result);
}

fn main() {
    let num1 = MyNumber(5);
    let num2 = MyNumber(10);

    let sum = num1 + num2;
    println!("Sum: {:?}", sum);

    let num3 = MyNumber(7);
    let added_num = num3.add_self();
    println!("Added num: {:?}", added_num);

    call_addable(num3);
}

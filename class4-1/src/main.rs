/////////////////////////////////////////////////////////
///////////               enum            ///////////////
/////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////
// 使用枚举：
// 在这种方法中，使用一个枚举来显式地包装不同类型，然后通过模式匹配在 MyEnum 的方法中分派调用。
// 优点：类型的调用是直接的，不涉及动态分发。编译器可以优化调用，因为在编译时就已经知道要调用的方法。
// 缺点：需要提前知道所有可能的类型，并在枚举中定义。

struct TypeA;
struct TypeB;
struct TypeC;

impl TypeA {
    fn do_something(&self) {
        println!("TypeA xxxxx");
    }
}

impl TypeB {
    fn do_something(&self) {
        println!("TypeB xxxxx");
    }
}

impl TypeC {
    fn do_something(&self) {
        println!("TypeC xxxxx");
    }
}

impl MyEnum {
    fn do_something(&self) {
        match self {
            MyEnum::TypeA(type_a) => type_a.do_something(),
            MyEnum::TypeB(type_b) => type_b.do_something(),
            MyEnum::TypeC(type_c) => type_c.do_something(),
        }
    }
}

enum MyEnum {
    TypeA(TypeA),
    TypeB(TypeB),
    TypeC(TypeC),
}

fn eunm_main() {
    let items: Vec<MyEnum> = vec![
        MyEnum::TypeA(TypeA),
        MyEnum::TypeB(TypeB),
        MyEnum::TypeC(TypeC),
    ];

    for item in items {
        item.do_something();
    }
}

/////////////////////////////////////////////////////////
////////////////         trait        ///////////////////
/////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////
// 使用Trait对象：
// 在这种方法中，我们使用Trait对象（Box<dyn MyTrait>）来存储不同类型，通过动态分发来调用方法。
// 优点：不需要事先知道所有可能的类型，可以在运行时决定使用哪个具体类型。
// 缺点：涉及动态分发，可能导致轻微的性能开销，因为调用方法是在运行时解析的。

trait MyTrait {
    fn do_something(&self);
}

struct TypeAA;
struct TypeBB;
struct TypeCC;

impl MyTrait for TypeAA {
    fn do_something(&self) {
        println!("TypeA is  xxxxx");
    }
}

impl MyTrait for TypeBB {
    fn do_something(&self) {
        println!("TypeB is  xxxxx");
    }
}

impl MyTrait for TypeCC {
    fn do_something(&self) {
        println!("TypeC is  xxxxx");
    }
}

fn trait_main() {
    let items: Vec<Box<dyn MyTrait>> = vec![Box::new(TypeAA), Box::new(TypeBB), Box::new(TypeCC)];

    for item in items {
        item.do_something();
    }
}

fn main() {
    eunm_main();
    trait_main();
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point3D(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn is_bigger_than(&self, other: &Rectangle) -> bool {
        self.calculate_area() > other.calculate_area()
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[derive(Debug)]
struct Unit {}

fn main() {
    user_test();

    rect_test();

    square_test();

    unit_test();

    color_and_point();
}

fn square_test() {
    let sq = Rectangle::square(1000);

    println!("{:#?}", sq);
}

fn rect_test() {
    let width = 100;
    let height = 200;

    let rect = Rectangle { width, height };

    let area = rect.calculate_area();

    println!("area: {}", area);

    let mut rect2 = Rectangle { width: 300, ..rect };

    println!("area of rect2: {}", rect2.calculate_area());

    rect2.double();

    println!("{:#?}", rect2);

    println!("area of rect2: {}", rect2.calculate_area());

    let rect_new_1 = Rectangle {
        width: 10,
        height: 20,
    };

    let rect_new_2 = Rectangle {
        width: 40,
        height: 60,
    };

    let rect_new_3 = Rectangle {
        width: 50,
        height: 100,
    };

    let rect_new_4 = Rectangle {
        width: 10,
        height: 22300,
    };

    println!(
        "rect1 can contain rect2: {}",
        rect_new_1.can_contain(&rect_new_2)
    );

    println!(
        "rect1 can contain rect3: {}",
        rect_new_1.can_contain(&rect_new_3)
    );

    println!(
        "rect3 can contain rect2: {}",
        rect_new_3.can_contain(&rect_new_2)
    );

    println!(
        "rect4 can contain rect2: {}",
        rect_new_4.can_contain(&rect_new_2)
    );

    println!(
        "rect4 is bigger than rect2: {}",
        rect_new_4.is_bigger_than(&rect_new_2)
    );
}

fn user_test() {
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("hello world"),
        active: true,
        sign_in_count: 2,
    };

    user.email = String::from("abiria.abiria@gmail.com");

    println!("{:#?}", user);

    let user2 = build_user(String::from("asd@asd.asd"), String::from("qwe@qwe.qwe"));

    println!("{:#?}", user2);
}

fn color_and_point() {
    let color = Color(21, 334, 54);
    let point = Point3D(23312, 3442, 234_432);

    println!("{:?}", color);
    println!("{:#?}", point);
}

fn unit_test() {
    let unit = Unit {};

    println!("{:?}", unit);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

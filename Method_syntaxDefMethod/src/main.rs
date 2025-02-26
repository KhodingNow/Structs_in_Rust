fn main() {
    
    // Method Syntax in Rust - Define Methods:

     #[derive(Debug)]
     struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    

    //fn main() {
    //     let rect1 = Rectangle {
    //         width: 30,
    //         height: 50,
    //     };

    //     println! (
    //         "The area of the rectangle is {} square pixels",
    //         rect1.area()
    //     );
    // //}
    
    // // Giving Method the same name one of the struct's fields - def method on Rectangle that is also named with:

    // impl Rectangle {
    //     fn width(&self) -> bool {
    //         self.width > 0 
    //     }
    // }

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // if rect1.width() {
    //     println!("The rectangle has a nonzero width; it is {}", rect1.width);
    // }
    

    // Methods with parameters

    let rect1 = Rectangle {
        width: 80,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}





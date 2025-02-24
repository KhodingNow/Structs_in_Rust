fn main() {
    // Structs are like Tuples - in that they hold multiple related values. Like Tuples, the pieces of a struct can be different types. Ulike with a Tuples, in  struct you will name each piece of data so it is clear what each value mean. 
    // adding the names means structs are more flexible than tuples - you don't have to rely on the order of the data to specify or access the values of an instance
    
    // Struct defined:

    //  struct User {
    //      active: bool,
    //      username: String,
    //      email: String,
    //      sign_in_count: u64,
    //  }

    // // Using a Struct:

    // let _user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("toto@example.com"),
    //     sign_in_count: 1
    // };

    // // Getting a specific vaalue from a struct

    // let mut user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("toto@example.com"),
    //     sign_in_count: 1,
    // };

    // user1.email = String::from("anotheremail@example.com");

    // fn build_user(email: String, username: String) -> User {
    //     User {
    //         active: true,
    //         username: username,
    //         email: email,
    //         sign_in_count: 1,
    //     }
   // }
    // Using the Field Init Shorthand

    // fn build_user(email: String, username: String) -> User {
    //     User {
    //         active: true,
    //         username,
    //         email,
    //         sign_in_count: 1,
    //     }
    // }


    // Creating Instances from Other Instances with Struct Update Syntax

    //fn main() {
        // let mut user1 = User {
        //     active: true,
        //     username: String::from("someusername123"),
        //     email: String::from("someone@example.com"),
        //     sign_in_count: 1,
        // };
    
        // user1.email = String::from("anotheremail@example.com");
    //}

    // let _user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // OR..

    // let _user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    // Core Roles of Structs

    // A Data Modelling
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    // Associated Functions
    impl Rectangle {
        // Constructor
        fn square(size: u32) -> Self {
            Rectangle { width: size, height: size}
        }
    }

    // Usage
    let sq = Rectangle::square(12);

    // Advanced Struct Types

    // A Tuple Structs

    struct Colour(u8, u8, u8);
    let _black = Colour(0, 0, 0);

    // Structs and Ownerhip (references with lifetmes)

    // struct User {
    //     username: String,
    // }
    // struct UserView<'a> {
    //     username: &'a str, 
    // }

    // Pattern Matching with Structs

    struct Point {
        x: i32,
        y: i32,
    }

    fn print_coordinates(p: Point) {
        match p {
            Point {x,y} => println!("({}, {})", x,y),
        }
    }

    // State MAchines
    enum VendingMachineState {
        Idle, 
        Processing,
        Dispensing,
    }
    struct VendingMachine {
        state: VendingMachineState,
        balance: u32,
    }

    // Deriving Traits for Structs

    #[derive(Debug, Clone, PartialEq)]
    struct User {
        id: u32,
        name: String,
    }

    // Practical Struct - Real World

    #[derive(Debug)]
    struct BlogPost {
        title: String,
        content: String,
        views: u32,
        tags: Vec<String>,
    }

    impl BlogPost {
        // Constructor
        fn new(title: String, content: String) -> Self {
            BlogPost {
                title,
                content,
                views: 0,
                tags: Vec::new(),
            }
        }

        // Method to increment views
        fn increment_views(&mut self) {
            self.views += 1;
        }

        fn add_tag(&mut self, tag: String) {
            self.tags.push(tag);
        }
    }

    fn main() {
        let mut post = BlogPost::new(
            String::from("Rust Structs Guide"),
            String::from("Learn Structs please"),

        );
        post.add_tag(String::from("Rust"));
        post.increment_views();
        println!("{:#?}", post);

    }



}

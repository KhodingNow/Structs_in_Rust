fn main() {
    // Structs are like Tuples - in that they hold multiple related values. Like Tuples, the pieces of a struct can be different types. Ulike with a Tuples, in  struct you will name each piece of data so it is clear what each value mean. 
    // adding the names means structs are more flexible than tuples - you don't have to rely on the order of the data to specify or access the values of an instance
    
    // Struct defined:

     struct User {
         active: bool,
         username: String,
         email: String,
         sign_in_count: u64,
     }

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

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }


    // Creating Instances from Other Instances with Struct Update Syntax

    //fn main() {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
    
        user1.email = String::from("anotheremail@example.com");
    //}

    // let _user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // OR..

    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };


}

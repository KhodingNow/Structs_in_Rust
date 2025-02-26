Defining Methods:

to define a functionwithin the context of a Rectangle, we start an impl (implementation) block for Rectangle. Everything within this impl block will be associated with the Rectangle type. Then, we move the area function within the impl curly brackets and change the first parameter to be self in the signature and everywhere within the body. In main, where we called the area function and passed rect1 as an argument, we can instead use method syntax to call the area method on our REctangle instance. 
The method syntax goes after an instance: we add a dot followed by the method name, brackets, and any arguments.
 In the signature of the area, we use &self instead of rectangle: Rectangle. The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for. Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with a only the name self shorthand to indicate that this method borrows the Self instance, just as we did in rectangle: &Rectangle.
 Methods can take ownership of self, borrow self immutably, as we did here., or borrow self mutably, just as they can any other parameter.

 We chose &self here because we dont want to take ownership - we just want to read the data in the struct, not write to it.
 If we wanted to change the instance that we've called the method on as part of what the method does, we'd use &mut self as the first parameter - this is RARE but this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.

 The main reason for for using METHODS instead of FUNCTIONS, in addition to providing method syntax and not having to repeat the type of self in every method's signature, is for organization.    

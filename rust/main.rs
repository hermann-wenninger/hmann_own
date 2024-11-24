fn main(){
    struct User<'alpha>{
        name:& 'alpha str,
        age: u8,
        email:& 'alpha str, 

    }
 struct Tweet <'alpha>{
    user: & 'alpha  User<'alpha>,
    content : & 'alpha str,
 }
 impl <'alpha> User <'alpha>{
    fn newUsr(name: & 'alpha str, age: u8, email: & 'alpha str) -> User <'alpha>{
        User{
            name,
            age,
            email,
        }
    }
 }
impl <'alpha> Tweet <'alpha>{
    fn new(user: & 'alpha User<'alpha>, content: & 'alpha str) -> Tweet <'alpha>{
        Tweet{
            user,
            content,
        }
    }
}
    let user = User{
        name: "John",
        age: 23,
        email: "
}




    }
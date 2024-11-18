fn main(){
    let a = "hello1";
    println!("{}", a);
    {
        let a = "hello2";
        println!("{}", a);
        
    }
    println!("{}", a);
}
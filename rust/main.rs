fn main(){
    
    let x = calc_string_len("hello".to_string());
    println!("{}{}", x.0, x.1);
    let a = "hello1".to_string();
    println!("{}", a);
    {
        let a = "hello2";
        println!("{}", a);
        
    }
    println!("{}", a);

    fn calc_string_len(s:String) ->(String, usize){
        let l = s.len();
        (s, l)
    }
    let y = calc_ref_len("hellooooo");
    println!("hello{}", y);
    fn calc_ref_len(s:&str) ->usize{
         s.len()
      }
      let mut s = "helloaloha".to_string();
      fn change_string(s: &mut String){
          s.push_str("  ...alohaworld");
      }
        change_string(&mut s);
        println!("{}", s);
}
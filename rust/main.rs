fn main(){
    use std::thread;

        println!("Hello from the main thread ....... 1.");
       let t1 = thread::spawn(fd);
        println!("Hello from the main thread ........2.");
      let t2 = thread::spawn(fd);
        println!("Hello from the main thread ........3.");
      let t3 =  thread::spawn(fd);
        println!("Hello from the main thread ........4.");
      let t4 =  thread::spawn(fd);
        println!("Hello from the main thread ........5.");
      let t5 =  thread::spawn(fd);
        println!("Hello from the main thread ........6.");
      let t6 =  thread::spawn(fd);
        println!("Hello from the main thread ........7.");
       let t7 = thread::spawn(fd);
        println!("Hello from the main thread ........8.");
       let t8 = thread::spawn(fd);
        
    
        println!("Hello from the main thread ........end.");
    
    
    fn fd() {
        println!("Hello from another thread!");
    
        let id = thread::current().id();
        println!("This is my thread id: {id:?}");
    }
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
    t5.join().unwrap();
    t6.join().unwrap();
    t7.join().unwrap();
    t8.join().unwrap();
    }
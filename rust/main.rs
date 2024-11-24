fn main(){
    use std::thread;

        println!("Hello from the main thread ....... 1.");
        thread::spawn(fd);
        println!("Hello from the main thread ........2.");
        thread::spawn(fd);
        println!("Hello from the main thread ........3.");
        thread::spawn(fd);
        println!("Hello from the main thread ........4.");
        thread::spawn(fd);
        println!("Hello from the main thread ........5.");
        thread::spawn(fd);
        println!("Hello from the main thread ........6.");
        thread::spawn(fd);
        println!("Hello from the main thread ........7.");
        thread::spawn(fd);
        println!("Hello from the main thread ........8.");
        thread::spawn(fd);
        
    
        println!("Hello from the main thread ........end.");
    
    
    fn fd() {
        println!("Hello from another thread!");
    
        let id = thread::current().id();
        println!("This is my thread id: {id:?}");
    }

    }
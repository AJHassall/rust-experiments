    pub struct Timer{
        start: std::time::Instant,
    }
    
    impl Drop for Timer{
        fn drop(&mut self){
            let end = std::time::Instant::now();
            println!("Duration: {:?}", end - self.start);
        }
    }
    impl Timer {
        pub fn new() -> Timer {
            Timer
            {
                start: std::time::Instant::now()
            }
        }
    }
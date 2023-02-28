use std::sync::{Arc, Mutex};            
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));     //wrapped up the value inside a mutex
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            
            //we wanted to mutate the value inside Arc<> so we wrapped up the value inside Mutex<>
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
        
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
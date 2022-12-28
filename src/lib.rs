pub mod ServerThread
{   
    use std::sync::mpsc::Sender;
    use std::thread::JoinHandle;
    use std::{sync::{mpsc, Arc, Mutex}, thread};

    type Job = Box<dyn FnOnce() + Send + 'static>;
    pub struct ThreadPool{
        workers : Vec<Worker>,
        sender : Option<Sender<Job>>
    }

    impl ThreadPool {
        pub fn new( size : usize) -> ThreadPool{
            assert!( size > 0);

            let (sender, receiver) = mpsc::channel::<Job>();

            let mut workers = Vec::with_capacity(size);

            let Reseiver = Arc::new(Mutex::new(receiver));

            for i in 0..size{
                workers.push( Worker::new(i, Arc::clone(&Reseiver)));
            }

            ThreadPool{ workers, sender: Some(sender) } 
        }

        pub fn execut<F>(&self, f : F)
        where 
            F : FnOnce() + Send + 'static
        {
            self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
        }

    }

    impl Drop for ThreadPool {

        fn drop(&mut self){

            drop(self.sender.take());

            for worker in &mut self.workers{

                println!("Shutting down worker {}", worker.id);

                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            } 

        }
    }

    struct Worker {
        id : usize,
        thread : Option<JoinHandle<()>>,
    }

    impl Worker {

        fn new( Id : usize, receiver : Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{

            let newThread = thread::spawn(move || loop {
                    let job = receiver.lock().unwrap().recv();
                    match job {
                        Ok(job) => {
                            println!("Worker {Id} got a job; executing.");
                            job();
                        },
                        Err(_) => {
                            println!("Worker {Id} disconnected; shutting down.");
                            break;
                        }

                    }
                    
                });

            Worker { 
                id : Id,
                thread : Some(newThread),
            }

        }

    }


    

}
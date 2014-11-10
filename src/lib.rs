use std::comm::{channel,SyncSender,Sender};

pub enum DoTask {
    Quit,
    Work(proc():Send),
}

struct Worker;
impl Worker {
    pub fn new (tx:SyncSender<SyncSender<DoTask>>) {
        let tb = std::task::TaskBuilder::new();
        let tb = tb.named("worker");
        tb.spawn(proc() {
            let (wx, wr) = sync_channel::<DoTask>(1);
            tx.send(wx.clone()); //send taskman our worker chan

            loop { //wait on work
                match wr.recv() {
                    Work(f) => {
                        f();
                        tx.send(wx.clone())
                    }
                    _ => break,
                }
            }
        });
    }
}

pub struct TaskMan {
    tx:SyncSender<SyncSender<DoTask>>, //task sender
    fx:Sender<DoTask>, //func sender
}

impl TaskMan {
    pub fn new (n:u8) -> TaskMan {// fr:Receiver<DoTask>, Sender<Sender<DoTask>>
        let (fx, fr) = channel::<DoTask>();
        let (tx,tr) = sync_channel::<SyncSender<DoTask>>(0);
        let tb = std::task::TaskBuilder::new();
        let tb = tb.named("taskman");
        tb.spawn(proc() {
            loop {
                match fr.recv() {
                    Quit => {println!("taskman quit!");
                             break;},
                    Work(f) => tr.recv().send(Work(f)),
                }
            }
        });
        
        let tm = TaskMan{tx:tx,fx:fx};
        tm.spawn_n(n); //spawn atleast n workers
        tm
    }

    pub fn spawn (&self) {
        Worker::new(self.tx.clone());
    }
    pub fn spawn_n (&self, n:u8) {
        for _ in range(0,n-1) {self.spawn();}
    }

    pub fn send (&self, f:DoTask) {
        self.fx.send(f);
    }
}



#[test]
fn it_works() {
}

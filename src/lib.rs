use std::comm::{channel,SyncSender,Sender};
use std::sync::Arc;
use std::sync::atomic::{AtomicInt, Relaxed};

pub enum DoTask {
    Quit,
    Work(proc():Send),
}

struct Worker;
impl Worker {
    pub fn new (tx:SyncSender<SyncSender<DoTask>>,wc:Arc<AtomicInt>) {
        let tb = std::task::TaskBuilder::new();
        let tb = tb.named("worker");
        tb.spawn(proc() {
            let (wx, wr) = sync_channel::<DoTask>(1);
            tx.send(wx.clone()); //send taskman our worker chan

            loop { //wait on work
                match wr.recv() {
                    Work(f) => {
                        wc.fetch_sub(1,Relaxed);//sub count incase we panic
                        f();
                        wc.fetch_add(1,Relaxed);//increment count again, we're good
                        tx.send(wx.clone());//send taskman our availability
                    }
                    _ => break,
                }
            }
            wc.fetch_sub(1,Relaxed);
        });
    }
}

pub struct TaskMan {
    tx:SyncSender<SyncSender<DoTask>>, //task sender
    fx:Sender<DoTask>, //func sender
    wn:Arc<AtomicInt>, //number of workers
    wc:Arc<AtomicInt>, //current worker count
}

impl TaskMan {
    pub fn new (n:int) -> TaskMan {// fr:Receiver<DoTask>, Sender<Sender<DoTask>>
        let (fx, fr) = channel::<DoTask>();
        let (tx,tr) = sync_channel::<SyncSender<DoTask>>(0);

        let tm = TaskMan{tx:tx,fx:fx,wn:Arc::new(AtomicInt::new(n)),wc:Arc::new(AtomicInt::new(0))};

        let tm2 = tm.clone();

        let tb = std::task::TaskBuilder::new();
        let tb = tb.named("taskman");

        tb.spawn(proc() {
            loop {
                match fr.recv() {
                    Quit => break,
                    Work(f) => {
                        if tm2.wc.load(Relaxed) < tm2.wn.load(Relaxed) {
                            tm2.spawn_n(tm2.wn.load(Relaxed) - tm2.wc.load(Relaxed)); //spawn atleast n workers
                        }
                        tr.recv().send(Work(f));
                    }
                }
            }
        });
        tm
    }

    pub fn spawn (&self) {
        Worker::new(self.tx.clone(),self.wc.clone());
        self.wn.fetch_add(1,Relaxed); //update num of workers
        self.wc.fetch_add(1,Relaxed); //update num of workers
    }
    pub fn spawn_n (&self, n:int) {
        for _ in range(0,n-1) {self.spawn();}
    }

    pub fn send (&self, f:DoTask) {
        self.fx.send(f);
    }

    pub fn clone (&self)->TaskMan {
        TaskMan{
            tx:self.tx.clone(),
            fx:self.fx.clone(),
            wn:self.wn.clone(),
            wc:self.wc.clone()
        }
    }
}



#[test]
fn it_works() {
}

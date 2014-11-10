use std::comm::{channel,Sender,Receiver};


enum DoTask {
    Quit,
    Work(proc():Send), 
}

fn spawn_worker (tx:Sender<Sender<DoTask>>) {
    spawn(proc() {
        let (wx, wr) = channel::<DoTask>();
        tx.send(wx.clone()); //send taskman our worker chan
        //todo: arc add worker to taskman
        loop { //wait on work
            match wr.recv() {
                Quit => {println!("quit!");
                         break;},
                Work(f) => {f();
                            tx.send(wx.clone());}, //let taskman know we're waiting for more work
            }
        }
        //todo: arc remove worker from taskman
        });
}

fn spawn_taskman (fr:Receiver<DoTask>) -> Sender<Sender<DoTask>> {
    let (tx,tr) = channel::<Sender<DoTask>>();
    spawn(proc() {
        loop {
            match fr.recv() {
                Quit => {println!("quit!");
                         break;},
                Work(f) => tr.recv().send(Work(f)),
            }
        }
    });
    tx
}

//fn send_work (f:DoTask,tch

fn main (){
    let w = Work(proc() {println!("hi!");});
    let w2 = Quit;
    let (fx, fr) = channel::<DoTask>();
    
    let tx = spawn_taskman(fr);
    spawn_worker(tx.clone());
    fx.send(w);
    fx.send(w2);
}

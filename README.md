``` rust
extern crate "rust-taskman" as taskman;
use taskman::{TaskMan,Work,Quit};


fn main (){
    let w = Work(proc() {println!("{}", 1i/0i);});
    let w2 = Work(proc() {println!("hi!");});
    
    let tm = TaskMan::new(3);
    tm.send(w);
    tm.send(w2);
    tm.send(Quit);
}
```
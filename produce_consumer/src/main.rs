use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicOption, SeqCst, AtomicPtr};
use std::io::timer::Timer;
use std::time::Duration;

type Stats = HashMap<String, int>;

/*
struct Item {
	name:String,
	stats:Option<Stats>
}
fn make_item() -> Box<Item> {
	let mut h:HashMap<String, int> = HashMap::new();
	h.insert("health".to_string(), 10);
	return box Item { name:"hi".to_string(), 
	              stats: Some(h)
	            }
}
*/
//fn getStat<'r>(i:Item, stat:String) -> &'r int {
//	&i.stats.get(&stat);
//}


fn main() {
	
    struct BigObject { val:int };
    let mut old = box BigObject { val: 5 };
    let mut newb = box BigObject { val: 100 };

    let (ctx, crx) = channel();
    let (ptx, prx) = channel();


    let ptx2 = ptx.clone();
    let ctx2 = ctx.clone();
    // Producer
    spawn(proc() {
    	let mut buffer:Box<BigObject>; //p
    	let mut timer = Timer::new().unwrap();
    	while(true) {
    		buffer = prx.recv();
    		buffer.val = buffer.val + 1;
    		println!("producing {0}", buffer.val );
    		timer.sleep(Duration::milliseconds(2000));

    		ctx2.send(buffer);
    	}
    });

    //Consumer
    let mut timer2 = Timer::new().unwrap();
    spawn(proc() {
    	let mut buffer:Box<BigObject>; //c
    	//let mut old:Option<Box<BigObject>;
    	
    	while(true) {
    		buffer = crx.recv();
    		println!("consuming {0}", buffer.val );
    		
    		timer2.sleep(Duration::milliseconds(4000));

    		ptx2.send(buffer);
    	}
    });

    ptx.send(old);
    ptx.send(newb);

/*
    let shared_big_object = Arc::new(AtomicOption::new(old));

    //old.val = 10;


    let shared_big_object_clone = shared_big_object.clone();
    //shared_big_object.swap(old, SeqCst);

    spawn(proc() {
    	let unwrapped_big_object = shared_big_object_clone.take(SeqCst);
    	//while(unwrapped_big_object.is_some()) {
        	    println!("got a big object from another task {0}", unwrapped_big_object.unwrap().val );
    	//}
    	println!("exit");
    });*/
/*
    let shared_big_object_clone2 = shared_big_object.clone();


    spawn(proc() {
    	let unwrapped_big_object = shared_big_object_clone.take(SeqCst);
    	while(unwrapped_big_object.is_some()) {
        	    println!("got a big object from another task");
    	}
    	println!("exit");
    });
*/

    //	shared_big_object.swap(old, SeqCst);
	//shared_big_object.swap(null, SeqCst);

    
}

fn main2() {
	/*
	let mut i = make_item();


	match i.stats {
		Some(ref v) => {
			for(ref name,ref value) in v.iter() {
				println!("{} {}", name, value)
			}
		}
		None => {}
	}

	println!("The program \"{}\" calculates the value {}",
              i.name, i.stats);
*/
    // A simple integer calculator:
    // `+` or `-` means add or subtract by 1
    // `*` or `/` means multiply or divide by 2
/*
    let program = "+ + * - /";
    let mut accumulator = 0i;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { }
        }
    }

    println!("The program \"{}\" calculates the value {}",
              program, accumulator);
    */
}
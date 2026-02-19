use num_cpus;
use std::thread;

fn main() {
    let cores = num_cpus::get();
    let value = 42;
    let mut handles = vec![];
    // Get a reference to the value and print its address using "{:p}"
    println!("The address of the value is: {:p}", &value);

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} running!", i);
        });
        handles.push(handle);
    }

    println!("Spawned {} threads", handles.len());

    for h in handles {
        h.join().unwrap();
    }

    // logging cores
    println!("number of cores: {cores}");

    let mut data = String::from("Hello, world!");
    // You can also get a raw pointer using the std::ptr::addr_of! macro for more control
    let data_address: *const String = std::ptr::addr_of!(data);
    println!("The address of the String struct is: {:p}", data_address);


    // To get the address of the actual string data on the heap, use as_ptr()
    println!("The address of the heap data is: {:p}", data.as_ptr());

    let x = 42;
    let r = &x;

    println!("x address: {:p}", &x);
    println!("r points to: {:p}", r);
    println!("r address (where reference lives): {:p}", &r);
}

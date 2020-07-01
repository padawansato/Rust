static mut v:Option<Vec<i32>>=None;
unsafe {
    v=Some(vec![1,2,3]);
    println!("{:?}",v);
}
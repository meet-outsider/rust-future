pub fn ownership() -> bool {
    let mut origin = String::from("yes");
    println!("is origin variable has this power ? {}", origin);
    let end = "yes".to_owned();
    change(&mut origin);
    // end = "sss";
    println!("is origin variable has this power ? {}", origin);
    println!("end end variable borrow this power ? {}", end);
    true
}

fn change(param: &mut String) {
    param.clear();
    param.push_str("after")
}

 fn main() {

    let a = &"Fred";
    let x: &str;
    let b: &str;
    {
        b = &"Fred2";
        x = own_and_change(a, b, true);

    }
    println!("{:?}, {:?}", a, x);


}

fn own_and_change<'a>(p1: &'a str, p2: &'a str, p3: bool)  -> &'a str {
    if p3 {
        p1
    } else {
        p2
    }
}
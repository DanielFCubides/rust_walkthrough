// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Scroll down for hints :)

pub fn ex3() {
    println!("ex3");
    let mut vec0 = Vec::new();

    fill_vec3(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);

}

fn fill_vec3(vec: &mut Vec<i32>){
    vec.push(22);
    vec.push(44);
    vec.push(66);

}

















// The difference between this one and the previous ones is that the first line
// of `fn fill_vec` that had `let mut vec = vec;` is no longer there. You can,
// instead of adding that line back, add `mut` in one place that will change
// an existing binding to be a mutable binding instead of an immutable one :)

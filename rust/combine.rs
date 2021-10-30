use std::collections::HashMap;

fn main() {
    // combine two hashmaps with the same keys
    let mut h1 = HashMap::new();
    let mut h2 = HashMap::new();

    h1.insert(vec![1, 3], vec![1, 2, 3]);
    h1.insert(vec![1, 2], vec![1, 2, 3]);
    h2.insert(vec![1, 2], vec![2, 3, 4]);
    h2.insert(vec![3, 4], vec![2, 3, 4]);

    println!("{:?}", h1);
    println!("{:?}", h2);

    map_combine(h1, h2)

}

// for combine - create a map h3 with all unique keys of h1 and h2 and empty tuples
// for each map h1 and h2, insert value into tuple for each key in h3
fn map_combine(h1: HashMap<Vec<i32>, Vec<i32>>, h2: HashMap<Vec<i32>, Vec<i32>>) {

    let mut h3 = h1.clone();

    // this will create a tuple with all the right keys but the wrong (overwritten values)
    h3.extend(h2);

    let mut h4: HashMap<Vec<i32>, (Vec<i32>, Vec<i32>)> = HashMap::new();

    for (k, _v) in h3.clone().iter() {
        //println!("{:?}", h1[k]);
        h4.insert(k.to_vec(), (h1[k], h2[k]));
    }
    println!("{:?}", h4);

}

use std::fs;
use fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

mod combine;

fn main() {
    let paths = fs::read_dir("./data").unwrap();

    // A hash map to store raw inputs of user & friend - could implement this as a type later
    let mut user_friends = Vec::new();

    for path in paths {
        let path_str = &path.as_ref().unwrap().path().display().to_string();
        let friends = read_file(path_str);

        let user = path_str
            .replace("./data/", "")
            .replace(".csv", "")
            .parse::<i32>().unwrap();

        user_friends.push((user, friends));
    }

    let mut all_maps = Vec::new();

    // map this key value store
    for user in user_friends {
        all_maps.push(map(user));
    }
    //println!("{:?}", all_maps);

    let combined_map = combine::combine(all_maps);

    // combined maps should have 2 entried per vector
    // because when a is a friend of b, b must be a friend of a

    for (k, v) in combined_map.iter() {
        let res = reduce(k.to_vec(), v.to_vec());
        println!("{:?}", res)
    }
    // combine

    // reduce


    // for each file - map it

    //read_file(paths.next());
}

fn reduce(k: Vec<i32>, friends: Vec<Vec<i32>>) -> (Vec<i32>, usize) {

    if friends.clone().len() == 1 {
        return (k, 0);
    } else {
        let s1: HashSet<i32> = friends[0].clone().into_iter().collect();
        let s2: HashSet<i32> = friends[1].clone().into_iter().collect();
        let mut intersection = s1.intersection(&s2);
        return (k, intersection.count())
    }

}

fn map(user: (i32, Vec<i32>)) -> HashMap<Vec<i32>, Vec<i32>> {
    let mut user_map = HashMap::new();
    for friend in user.1.clone() {
        let mut key = vec![user.0, friend];
        key.sort();
        user_map.insert(key, user.1.clone());
    }
    return user_map
}

fn read_file(path: &String) -> Vec<i32> {

    let mut file = File::open(&path).expect("Can't open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let split = contents.split("\n");

    let mut friends = vec![];

    // fill map value with vector of friends
    for s in split {
        friends.push(s.clone().parse::<i32>().unwrap())
    }
    return friends
}

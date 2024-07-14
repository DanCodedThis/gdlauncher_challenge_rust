use std::{collections::BTreeMap, fs::read_to_string};
fn main() {
    let big_string = read_to_string("./challenge_input.txt").expect("File was not fould.");
    let ints: Vec<i128> = big_string.lines().map(|item| {
        item.parse::<i128>().unwrap()
    }).collect();

    //why not a hash map? If you use hash map it will sometimes will have more elemtns in the map then it should (which is 100 at max), I didn't investigate it further
    // but the only differnce between the two that I know for certain is that BTreeMap is orderable and HashMap is not
    let mut ints_map: BTreeMap<i128, u8> = BTreeMap::new();
    //get the first hundred elemnts in to the map, even if they are not unique
    for i in 0..100 {
        let item = &ints[i];
        if let Some(num) = ints_map.get(item) {
            ints_map.insert(*item, num + 1);
        } else {
            ints_map.insert(*item, 1);
        }
    }
    //main algorithm
    let mut err_vals:Vec<i128> = vec![];
    //each iteration we get the next test item which start form 100 (because we test from 0-99)
    'outer: for i in 100..ints.len() {
        let first = ints[i - 100];
        let test = ints[i];
        //we clone because other wise we hav a mutabe and an imutable borrow at the same time
        let cloned_ints_map = ints_map.clone();
        'inner: for (_j, item) in cloned_ints_map.keys().enumerate() {
            //test is supposed to be bigger then the item to be considred "findable", if it is negative we wont find anything
            let what = test - *item;
            //then we check if the "what" is a number in the map if it is and it is not duplicate of itself without having atleats one copy of itself, for exmaple 2 + 2 = 4, 
            if let Some(num) = ints_map.get(&what) {
                if what == *item && *num == 1 {
                    continue 'inner;
                }
                //if we find such an item we forward the map, in the next iteration after the first we test from 1-100, and the test number is 101 position
                forward_map(&mut ints_map, &first, &test, &i);
                continue 'outer;
            }
        }
        //if we didnt find any possible number in the map, we again forward it and add it to the vector of the error values
        forward_map(&mut ints_map, &first, &test, &i);
        err_vals.push(test);
    }
    //at the end we get the length of the vector and its contents
    println!("{}", err_vals.len());
    println!("{:?}", err_vals);
}
//forwrding the map - first element + last element which is the test elemt 
fn forward_map(ints_map: &mut BTreeMap<i128, u8>, first: &i128, last: &i128, debug_i: &usize) {
    //remove old value if needed, otherwise decrement, the map will be at max at 100
    if let Some(num) = ints_map.get(&first) {
        if *num == 1 {
            ints_map.remove(&first);
        } else {
            ints_map.insert(*first, *num - 1); 
        }
    } else {
        //if you change to hashmap there is a chance you get this message, and the map can be bigger then 100 elements
        println!("bug first {} @ pos {} iter {} num 0", first, debug_i - 99, debug_i);
    }
    //add new value or incriment
    if let Some(num) = ints_map.get(&last) {
        ints_map.insert(*last, *num + 1);
    } else {
        ints_map.insert(*last, 1);
    }
}

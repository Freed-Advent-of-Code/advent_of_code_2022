use std::fs;

pub fn compartment_part_one () {

    let file = fs::read_to_string("./src/day_three/input.txt").unwrap();

    let list : Vec<String>  = file
        .lines()
        .map(|compartment| {
            let v = compartment.to_string();
            let string_length = v.len()/2;
            let (first, second)= v.split_at(string_length);
            for c in first.split("") {
                if second.contains(c) {
                    c.to_string()
                } else {
                    String::from("fff")
                }
            }


            // first.to_string()
            // first_part
            // let second_part = &v[*&string_length/2..*&string_length];
            // first_part
        })
        .collect();

    println!("{:?}", list);

}
use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let file_contents = read_to_string("data.txt").unwrap();

    let mut queue: VecDeque<char> = VecDeque::from(['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a']);

    let mut first_occurance = 0;

    for (index, char) in file_contents.chars().enumerate() {
        queue.push_back(char);
        queue.pop_front();

        let mut duplicate = false;

        for i in 0..14 {
            for j in (i + 1)..14 {
                if queue[i] == queue[j] {
                    duplicate = true;
                }
            }
        }

        println!("{:?} : {:?}", queue, duplicate);

        if index >= 13 {
            if !duplicate {
                first_occurance = index;
                break;
            }
        }
    }

    println!("{}", first_occurance + 1);
}

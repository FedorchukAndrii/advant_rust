use std::str::SplitWhitespace;

// dir_path is an array of strings ["/", "dir1", "dir2" ... ], where each string is a name of a directory that is same as "./dir1/dir2/..."
static mut DIR_PATH: Vec<String> = Vec::new();

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}



pub fn part_one(input: &str) -> Option<u32> {
    // Return a sum of all dir weights, that are lower than 100_000
    for line in input.lines() {
        parse_input_line(line);
    }
    // let mut sum = 0;
    // for dir in dirs.iter() {
    //     if dir.weight < 100_000 {
    //         sum += dir.weight;
    //     }
    // }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    println!("second part");
    None
}

fn parse_input_line(line: &str)  {
    
    // Parse a line of input. If first char is a $ then it's command, if first word "dir" then it's a directory, if first word contains only numbers then it's a size of a file and second word is a name of a file.
    let mut words = line.split_whitespace();
    let first_word = words.next().unwrap();
    let second_word = words.next().unwrap();
    if first_word.starts_with("$") {
        // It's a command. if second word "cd" then it's a change directory command, if second word "ls" then it's a list directory command
        match second_word {
            "cd" => logic_after_cd_command(words),
            "ls" => println!("ls command"),
            _ => println!("Unknown command"),
        }
    } else if first_word == "dir" {
        // It's a directory
        println!("Directory: {}", second_word);
    } else {
        // It's a file
        println!("File: {}", first_word);
    }     
}

fn logic_after_cd_command(mut words: SplitWhitespace) {
    let third_word = words.next().unwrap();
    if third_word == ".." {
        // remove last element from dir_path, write safe function
        unsafe {
            DIR_PATH.pop();
            println!("path: {:?}", DIR_PATH.join("/"));
        }
        return;
    }
    let now_in_dir = String::from(third_word);
    unsafe {
        DIR_PATH.push(now_in_dir);
    }
    println!("Now in directory: {:?}", unsafe{DIR_PATH.get(DIR_PATH.len()-1)})
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}

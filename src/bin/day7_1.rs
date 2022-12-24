#[derive(Debug)]
enum CommandName {
    ChangeDir,
    LS,
}

#[derive(Debug)]
struct Command<'a> {
    name: CommandName,
    arg: Option<&'a str>,
}

impl<'a> Command<'a> {
    fn new(name: CommandName, arg: Option<&'a str>) -> Self {
        Self { name, arg }
    }

    fn from_str(s: &'a str) -> Result<Self, ()> {
        if s.starts_with("$ cd") {
            let target_dir = &s[5..];

            return Ok(Command::new(CommandName::ChangeDir, Some(target_dir)));
        } else if s.starts_with("$ ls") {
            return Ok(Command::new(CommandName::LS, None));
        }

        return Err(());
    }
}

fn main() {
    let input = include_str!("../inputs/day7.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    let max_dir_size = 100000;

    let mut stack = vec![("/", 0)];
    let mut total = 0;

    for line in lines {
        if let Ok(command) = Command::from_str(line) {
            match command.name {
                CommandName::ChangeDir => {
                    let target_dir = command.arg.unwrap();

                    if target_dir == "/" {
                        continue;
                    } else if target_dir == ".." {
                        let (_, amount) = stack.pop().unwrap();

                        if amount <= max_dir_size {
                            total += amount;
                        }

                        stack.last_mut().unwrap().1 += amount;
                    } else {
                        stack.push((target_dir, 0));
                    }
                }
                CommandName::LS => continue,
            }
        } else {
            let (amount, _) = line.split_once(" ").unwrap();

            if let Ok(amount) = amount.parse::<usize>() {
                stack.last_mut().unwrap().1 += amount;
            }
        }
    }

    println!("{}", total);
}

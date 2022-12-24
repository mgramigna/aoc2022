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

    let total_disk_space = 70000000;
    let needed_free_space = 30000000;

    let mut stack = vec![("/", 0)];
    let mut total_sizes: Vec<(&str, usize)> = vec![];

    for line in lines {
        if let Ok(command) = Command::from_str(line) {
            match command.name {
                CommandName::ChangeDir => {
                    let target_dir = command.arg.unwrap();

                    if target_dir == "/" {
                        continue;
                    } else if target_dir == ".." {
                        let (dir, amount) = stack.pop().unwrap();

                        stack.last_mut().unwrap().1 += amount;
                        total_sizes.push((dir, amount));
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

    while stack.len() > 0 {
        let (dir, amount) = stack.pop().unwrap();
        total_sizes.push((dir, amount));

        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += amount;
        }
    }

    let total_used_space: usize = total_sizes.iter().find(|i| i.0 == "/").unwrap().1;

    let unused_space = total_disk_space - total_used_space;

    let size_to_delete = total_sizes
        .iter()
        .filter(|(_, amount)| unused_space + amount >= needed_free_space)
        .map(|i| i.1)
        .min()
        .unwrap();

    println!("{}", size_to_delete);
}

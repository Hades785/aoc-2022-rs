use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Command {
    name: String,
    args: Option<Vec<String>>,
}

#[derive(Debug)]
struct CommandParseError;

impl FromStr for Command {
    type Err = CommandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"\$ (\w+)(?: (.+))?").unwrap();
        let captures = regex.captures(s).map(|captures| {
            captures
                .iter()
                .skip(1)
                .map(|c| c.map(|m| m.as_str()))
                .collect::<Vec<_>>()
        });

        match captures.as_ref().map(|c| c.as_slice()) {
            Some(&[Some(cmd), args]) => Ok(Command {
                name: cmd.to_string(),
                args: args.map(|a| a.split_whitespace().map(|arg| arg.to_string()).collect()),
            }),
            _ => Err(CommandParseError),
        }
    }
}

#[derive(Clone, Debug)]
struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    fn size(&self) -> u32 {
        let dir_size: u32 = self.directories.iter().map(|dir| dir.size()).sum();
        let file_size: u32 = self.files.iter().map(|file| file.size).sum();
        dir_size + file_size
    }
}

#[derive(Debug)]
struct DirectoryParseError;

impl FromStr for Directory {
    type Err = DirectoryParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"dir (.+)").unwrap();

        // Technically overkill, yeah, there can only be
        // two matches, the global match and the capture group.
        let captures = regex.captures(s).map(|captures| {
            captures
                .iter()
                .skip(1)
                .flatten()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
        });

        match captures.as_ref().map(|c| c.as_slice()) {
            Some(&[name]) => Ok(Directory {
                name: name.to_string(),
                directories: vec![],
                files: vec![],
            }),
            _ => Err(DirectoryParseError),
        }
    }
}

#[derive(Clone, Debug)]
struct File {
//    name: String,
    size: u32,
}

#[derive(Debug)]
struct FileParseError;

impl FromStr for File {
    type Err = FileParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"(\d+) (.+)").unwrap();
        let captures = regex.captures(s).map(|captures| {
            captures
                .iter()
                .skip(1)
                .flatten()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
        });

        match captures.as_ref().map(|c| c.as_slice()) {
            Some(&[size, _name]) => Ok(File {
//                name: name.to_string(),
                size: size.parse().unwrap(),
            }),
            _ => Err(FileParseError),
        }
    }
}

fn solve(input: &str) -> Vec<u32> {
    let mut root_dir = Directory {
        name: "/".to_string(),
        directories: vec![],
        files: vec![],
    };
    let mut current_dir = "/".to_string();

    for line in input.lines() {
        if line.starts_with("$") {
            let cmd = line.parse::<Command>().unwrap();
            match cmd.name.as_str() {
                "cd" => {
                    let args = cmd.args.unwrap();
                    let dir_name = args[0].as_str();
                    match dir_name {
                        ".." => {
                            current_dir = {
                                let split = current_dir.split("/");
                                let count = split.clone().count() - 1;
                                split.take(count).collect::<Vec<_>>().join("/")
                            }
                        }
                        name => current_dir = format!("{current_dir}/{name}").to_string(),
                    }
                }
                "ls" => {}
                _ => todo!("Unknown command {}", cmd.name),
            }
        } else {
            if let Ok(dir) = line.parse::<Directory>() {
                let mut working_dir = &mut root_dir;
                for fragment in current_dir.split("/").skip(1) {
                    working_dir = match working_dir
                        .directories
                        .iter_mut()
                        .find(|d| d.name == fragment)
                    {
                        Some(d) => d,
                        None => &mut root_dir,
                    };
                }
                working_dir.directories.push(dir);
            } else {
                let file = line.parse::<File>().unwrap();
                let mut working_dir = &mut root_dir;
                for fragment in current_dir.split("/").skip(1) {
                    working_dir = match working_dir
                        .directories
                        .iter_mut()
                        .find(|d| d.name == fragment)
                    {
                        Some(d) => d,
                        None => &mut root_dir,
                    };
                }
                working_dir.files.push(file);
            }
        }
    }

    let directories: &mut Vec<Directory> = &mut vec![];
    fn add_recursive(d: Directory, ds: &mut Vec<Directory>) {
        ds.push(d.clone());
        for dir in d.directories {
            let dir = dir.clone();
            add_recursive(dir, ds);
        }
    }
    add_recursive(root_dir, directories);

    directories.iter().map(|dir| dir.size()).collect()
}

fn solve_a(input: &str) -> u32 {
    solve(input).iter().filter(|s| s < &&100000).sum()
}

fn solve_b(input: &str) -> u32 {
    let sizes = solve(input);
    let total = *sizes.clone().iter().max().unwrap();
    let empty = 70000000 - total;
    let required = 30000000 - empty;

    *sizes.iter().filter(|s| s > &&required).min().unwrap()
}

fn main() {
    let input = include_str!("input_data/input.txt");
    println!("Part 1: {}", solve_a(input));
    println!("Part 2: {}", solve_b(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_a() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_a(input), 95437);
    }

    #[test]
    fn day07_b() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_b(input), 24933642);
    }
}

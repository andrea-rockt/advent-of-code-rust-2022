use std::{cell::RefCell, rc::Rc};

use itertools::Itertools;
use nom::Finish;

use crate::{parser::parse_commands, tree::TreeNode};

mod tree {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{DirNode, FileNode, FsNode};

    #[derive(PartialEq)]
    pub struct TreeNode<'a> {
        pub value: FsNode<'a>,
        pub children: Vec<Rc<RefCell<TreeNode<'a>>>>,
        pub parent: Option<Rc<RefCell<TreeNode<'a>>>>,
    }

    impl<'a> TreeNode<'a> {
        pub fn print(&self) -> String {
            let data = match self.value {
                FsNode::Dir(DirNode { name }) => name.to_string(),
                FsNode::File(FileNode { name, size }) => name.to_string() + "|" + &size.to_string(),
            };

            if self.children.is_empty() {}

            let mut s = String::new();
            s.push_str(&data);
            s.push('[');
            s.push_str(
                &self
                    .children
                    .iter()
                    .map(|tn| tn.borrow().print())
                    .collect::<Vec<String>>()
                    .join(","),
            );

            s.push(']');

            s
        }

        pub fn dir(name: &'a str) -> Rc<RefCell<TreeNode<'a>>> {
            Rc::new(RefCell::new(TreeNode {
                value: FsNode::Dir(DirNode { name }),
                children: vec![],
                parent: None,
            }))
        }

        pub fn file(name: &'a str, size: u32) -> Rc<RefCell<TreeNode<'a>>> {
            Rc::new(RefCell::new(TreeNode {
                value: FsNode::File(FileNode { name, size }),
                children: vec![],
                parent: None,
            }))
        }
    }
}

mod parser {

    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_while1};
    use nom::character::complete::{digit1, line_ending, space1};
    use nom::combinator::{map, map_res};
    use nom::error::VerboseError;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{preceded, terminated, tuple};
    use nom::IResult;

    use crate::{Commands, Definition};

    type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

    pub fn take_str(input: &str) -> Res<&str> {
        take_while1(|c| c != '\n')(input)
    }

    pub fn parse_cd_command(input: &str) -> Res<Commands> {
        map(parse_cd, |directory| Commands::Cd { directory })(input)
    }

    pub fn parse_ls_command(input: &str) -> Res<Commands> {
        map(parse_ls, |content| Commands::Ls { content })(input)
    }

    pub fn parse_cd(input: &str) -> Res<&str> {
        preceded(tag("$ cd "), take_str)(input)
    }

    pub fn parse_ls(input: &str) -> Res<Vec<Definition>> {
        map(
            tuple((
                terminated(tag("$ ls"), line_ending),
                separated_list1(line_ending, parse_ls_output),
            )),
            |d| d.1,
        )(input)
    }

    pub fn parse_dir(input: &str) -> Res<Definition> {
        map(preceded(tag("dir "), take_str), |x| Definition::Directory {
            name: x,
        })(input)
    }

    pub fn parse_file_line(input: &str) -> Res<Definition> {
        map(
            tuple((parse_unsigned_integer, preceded(space1, take_str))),
            |(size, name)| Definition::File { size, name },
        )(input)
    }

    pub fn parse_ls_output(input: &str) -> Res<Definition> {
        alt((parse_file_line, parse_dir))(input)
    }

    pub fn parse_unsigned_integer(input: &str) -> Res<u32> {
        map_res(digit1, |c: &str| c.parse::<u32>())(input)
    }

    pub fn parse_commands(input: &str) -> Res<Vec<Commands>> {
        many1(terminated(
            alt((parse_cd_command, parse_ls_command)),
            line_ending,
        ))(input)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum FsNode<'a> {
    Dir(DirNode<'a>),
    File(FileNode<'a>),
}
#[derive(Debug, PartialEq, Eq)]
pub struct DirNode<'a> {
    name: &'a str,
}
#[derive(Debug, PartialEq, Eq)]
pub struct FileNode<'a> {
    name: &'a str,
    size: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Definition<'a> {
    Directory { name: &'a str },
    File { size: u32, name: &'a str },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Commands<'a> {
    Cd { directory: &'a str },
    Ls { content: Vec<Definition<'a>> },
}

pub fn part_one(input: &str) -> Result<u32, String> {
    let (_, parsed) = parse_commands(input).finish().map_err(|e| e.to_string())?;

    let root: Rc<std::cell::RefCell<TreeNode>> = TreeNode::dir("/");

    build_tree(&root, parsed);

    let mut sum: u32 = 0;
    calculate_size(Rc::clone(&root), &mut sum)?;
    Ok(sum)
}

fn build_tree<'a>(root: &'a Rc<RefCell<TreeNode<'a>>>, parsed: Vec<Commands<'a>>) {
    let mut tree: Rc<std::cell::RefCell<TreeNode>> = Rc::clone(root);
    for command in parsed {
        match command {
            Commands::Cd { directory: "/" } => {}
            Commands::Cd { directory: ".." } => {
                let x = Rc::clone(&tree);
                let a = x.borrow_mut();
                match &a.parent {
                    Some(a) => {
                        tree = Rc::clone(a);
                    }
                    None => {}
                };
            }
            Commands::Cd { directory } => {
                let x = Rc::clone(&tree);
                let z = x.borrow();
                let b = &z.children;

                for c in b {
                    if c.borrow_mut().value == FsNode::Dir(DirNode { name: directory }) {
                        tree = Rc::clone(c);
                    }
                }
            }
            Commands::Ls { content } => {
                for c in content {
                    match c {
                        Definition::Directory { name } => {
                            let child = TreeNode::dir(name);
                            tree.borrow_mut().children.push(Rc::clone(&child));
                            {
                                let mut mut_child = child.borrow_mut();
                                mut_child.parent = Some(Rc::clone(&tree));
                            }
                        }
                        Definition::File { size, name } => {
                            let child = TreeNode::file(name, size);
                            tree.borrow_mut().children.push(Rc::clone(&child));
                            {
                                let mut mut_child = child.borrow_mut();
                                mut_child.parent = Some(Rc::clone(&tree));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn calculate_size(node: Rc<RefCell<TreeNode>>, sum: &mut u32) -> Result<u32, String> {
    let node = node.borrow();

    let mut size: u32 = 0;

    for c in &node.children {
        match c.borrow().value {
            FsNode::Dir(_) => {
                let _size = calculate_size(Rc::clone(c), sum)?;

                size += _size;
            }
            FsNode::File(FileNode { name: _, size: s }) => {
                size += s;
            }
        }
    }

    if size < 100000 {
        *sum += size;
    }

    Ok(size)
}

pub fn find_candidate(
    node: Rc<RefCell<TreeNode>>,
    candidates: &mut Vec<u32>,
) -> Result<u32, String> {
    let node = node.borrow();

    let mut size: u32 = 0;

    for c in &node.children {
        match c.borrow().value {
            FsNode::Dir(_) => {
                let _size = find_candidate(Rc::clone(c), candidates)?;

                size += _size;
            }
            FsNode::File(FileNode { name: _, size: s }) => {
                size += s;
            }
        }
    }

    candidates.push(size);

    Ok(size)
}

pub fn part_two(input: &str) -> Result<u32, String> {
    let (_, parsed) = parse_commands(input).finish().map_err(|e| e.to_string())?;

    let root: Rc<std::cell::RefCell<TreeNode>> = TreeNode::dir("/");

    build_tree(&root, parsed);

    let mut candidates: Vec<u32> = Vec::new();
    let used = find_candidate(Rc::clone(&root), &mut candidates)?;

    let total_space = 70000000;

    let free =  total_space - used;

    let to_be_freed = 30000000 - free;

    candidates
        .iter()
        .filter(|x| **x >= to_be_freed)
        .sorted()
        .next()
        .copied()
        .ok_or_else(|| "not_found".to_string())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_cd, parse_dir, parse_file_line, parse_ls, parse_ls_output};

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Ok(95437));
    }

    #[test]
    fn test_parse_ls() {
        let input = "$ ls
dir ddpgzpc
dir mqjrd
dir mrqjg
dir rglgbsq
298050 tjmjp.cqm
dir wlqhpwqv";

        let expected = Ok((
            "",
            vec![
                Definition::Directory { name: "ddpgzpc" },
                Definition::Directory { name: "mqjrd" },
                Definition::Directory { name: "mrqjg" },
                Definition::Directory { name: "rglgbsq" },
                Definition::File {
                    name: "tjmjp.cqm",
                    size: 298050,
                },
                Definition::Directory { name: "wlqhpwqv" },
            ],
        ));

        assert_eq!(
            parse_ls(input).finish().map_err(|x| x.to_string()),
            expected
        );
    }

    #[test]
    fn test_parse_cd_line() {
        let input = "$ cd directory";

        assert_eq!(
            parse_cd(input)
                .finish()
                .map_err(|x| x.to_string())
                .map(|x| x.1),
            Ok("directory")
        );
    }

    #[test]
    fn test_parse_dir_line() {
        let input = "dir directory_name";

        assert_eq!(
            parse_dir(input)
                .finish()
                .map_err(|x| x.to_string())
                .map(|x| x.1),
            Ok(Definition::Directory {
                name: "directory_name"
            })
        );
    }

    #[test]
    fn test_parse_file_line() {
        let input = "12 filename";

        assert_eq!(
            parse_file_line(input)
                .finish()
                .map_err(|x| x.to_string())
                .map(|x| x.1),
            Ok(Definition::File {
                size: 12,
                name: "filename"
            })
        );
    }

    #[test]
    fn test_parse_ls_output() {
        let input = "12 filename";
        let input1 = "dir directory";

        assert_eq!(
            parse_ls_output(input)
                .finish()
                .map_err(|x| x.to_string())
                .map(|x| x.1),
            Ok(Definition::File {
                size: 12,
                name: "filename"
            })
        );

        assert_eq!(
            parse_ls_output(input1)
                .finish()
                .map_err(|x| x.to_string())
                .map(|x| x.1),
            Ok(Definition::Directory { name: "directory" })
        );
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Ok(24933642));
    }
}

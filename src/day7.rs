use aoc22::{read_aoc_file, TreeNode};

#[derive(Clone, Debug)]
struct Dir{
    path: String,
    filesum: u64
}

fn main() {
    let mut current_dir = TreeNode::new_root(Dir {
        path: "/".to_owned(),
        filesum: 0
    });

    for line in read_aoc_file(7) {
        if line.starts_with("$ ") { // Command input
            let mut iter = line.split_ascii_whitespace().skip(1);
            let command = iter.next().expect("Got empty command input!");
            match command {
                "cd" => {
                    let target = iter.next().expect("Got empty cd command without argument!");
                    match target {
                        ".." => {
                            current_dir = current_dir.parent().expect("Trying to cd up when at root!");
                        },
                        "/" => {
                            current_dir = current_dir.get_root();
                        },
                        _ => {
                            for child in current_dir.children().iter() {
                                if child.value().path.ends_with(target) {
                                    current_dir = child.clone();
                                    break;
                                }
                            }
                        }   
                    };
                },
                "ls" => {},
                _ => panic!("Undefined command {}", command)
            };
        } else { // Command output
            let mut iter = line.split_ascii_whitespace();
            let output = iter.next().expect("Got an empty line!");

            if output == "dir" { // Got an directory entry
                let dir_name = iter.next().expect("Got directory with no name!");
                let path = match current_dir.value().path.as_str() {
                    "/" => format!("/{}", dir_name),
                    current_path => format!("{}/{}", current_path, dir_name)
                };

                current_dir.add_child(Dir {
                    path,
                    filesum: 0
                });
            } else if let Ok(num) = output.parse::<u64>() { // Got an file entry
                current_dir.value_mut().filesum += num;

                // Add file size to all parents
                let mut temp = current_dir.clone();
                while let Some(d) = temp.parent() {
                    temp = d;
                    temp.value_mut().filesum += num;
                }
            }
        }
    }

    // Got the complete file tree now
    let root = current_dir.get_root();
    let mut children: Vec<Dir> = Vec::new();
    collect_children(&root, &mut children);

    let sum: u64 = children.iter().filter_map(|dir| {
        if dir.filesum < 100_000 { 
            return Some(dir.filesum);
        } 
        None
    }).sum();
    println!("The added size of all small folders is {}", sum);

    // ################################## Part 2 #########################################

    let unused_space = 70_000_000 - root.value().filesum;
    let required_space = 30_000_000 - unused_space;

    let (path, size) = children.iter().filter_map(|value| {
        if value.filesum < required_space {
            return None;
        }
        Some((value.path.clone(), value.filesum))
    }).min_by_key(|temp| temp.1).unwrap();
    println!("Delete folder \"{}\", size {}", path, size);
}

fn collect_children(node: &TreeNode<Dir>, vec: &mut Vec<Dir>) {
    vec.push(node.value().clone());

    if node.has_children() {
        for child in node.children() {
            collect_children(&child, vec);
        }
    }
}
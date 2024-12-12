use std::collections::HashMap;

#[derive(Debug)]
enum Node<'a> {
    Dir(HashMap<&'a str, Node<'a>>),
    File(usize),
}
impl<'a> Node<'a> {
    fn size(&self, on_dir_size: &mut impl FnMut(usize)) -> usize {
        match self {
            Node::Dir(files) => {
                let total = files.iter().map(|(_, file)| file.size(on_dir_size)).sum();
                on_dir_size(total);
                total
            }
            &Node::File(size) => size,
        }
    }
}

pub fn run(input: &str) {
    println!("Day 7:");

    let mut d = vec![];
    let mut fs = Node::Dir(HashMap::new());
    let mut cur = &mut fs;
    
    for line in input.trim().lines() {
        match line {
            "$ cd /" => d.clear(),
            "$ cd .." => {
                d.pop().unwrap();
            }
            "$ ls" => {}
            _ if line.starts_with("$ cd ") => {
                d.push(&line[5..]);
            }
            _ if line.starts_with("dir ") => {
                let name = &line[4..];
                let mut cur = &mut fs;
                for s in d.iter().copied().chain(std::iter::once(name)) {
                    let Node::Dir(files) = cur else { panic!() };
                    cur = files.entry(s).or_insert(Node::Dir(HashMap::new()));
                }
            }
            _ => {
                let (size, name) = line.split_once(" ").unwrap();
                let mut cur = &mut fs;
                for s in d.iter().copied() {
                    let Node::Dir(files) = cur else { panic!() };
                    cur = files.entry(s).or_insert(Node::Dir(HashMap::new()));
                }
                let Node::Dir(dir) = cur else { panic!() };
                dir.insert(name, Node::File(size.parse().unwrap()));
            }
        }
    }
    // ---------- part 1 ----------
    let mut a = 0;
    let total_size = fs.size(&mut |size| if size <= 100000 { a += size });
    println!("\tPart 1: {}", a);
    // ---------- part 2 ----------
    let mut required = 30000000 - (70000000 - total_size);
    let mut del = usize::MAX;
    fs.size(&mut |size| if size >= required && size < del { del = size });
    println!("\tPart 2: {}", del);
}
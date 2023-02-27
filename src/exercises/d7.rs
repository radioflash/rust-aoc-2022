use std::borrow::BorrowMut;

use super::Part;

struct FsNode {
    name: String,
    entries: Vec<FsNode>,
    size: Option<u32>,
}

impl FsNode {
    fn new_dir(dirname: &str) -> FsNode {
        FsNode {name: String::from(dirname), entries: Vec::new(), size: None }
    }
    fn new_file(dirname: &str, size: u32) -> FsNode {
        FsNode {name: String::from(dirname), entries: Vec::new(), size: Some(size) }
    }

    fn add_directory(&mut self, name: &str) -> &mut FsNode {
        let entry = Self::new_dir(name);
        self.entries.push(entry);
        self.entries.last_mut().unwrap()
    }
    fn add_file(&mut self, name: &str, size: u32) -> &mut FsNode {
        let entry = Self::new_file(name, size);
        self.entries.push(entry);
        self.entries.last_mut().unwrap()
    }
    fn total_size(&mut self) -> u32 {
        match self.size {
            Some(x) => x,
            _ => {
                let mut tmp_size = 0;
                for sd in self.entries.iter_mut() {
                    tmp_size += sd.total_size();
                }
                self.size = Some(tmp_size);
                tmp_size
            },
        }
    }
}

pub fn solve(part: Part, input: &str) -> u32 {
    let mut root: FsNode = FsNode::new_dir("/");

    root.total_size()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn calculation() {
    // - / (dir)
    //   - a (dir)
    //     - e (dir)
    //       - i (file, size=584)
    //     - f (file, size=29116)
    //     - g (file, size=2557)
    //     - h.lst (file, size=62596)
    //   - b.txt (file, size=14848514)
    //   - c.dat (file, size=8504156)
    //   - d (dir)
    //     - j (file, size=4060174)
    //     - d.log (file, size=8033020)
    //     - d.ext (file, size=5626152)
    //     - k (file, size=7214296)
        let mut root = FsNode::new_dir("/");
        let a = root.add_directory("a");
        let e = a.add_directory("e");
        e.add_file("i", 584);

        a.add_file("f", 29116);
        a.add_file("g", 2557);
        a.add_file("h.lst", 62596);

        root.add_file("b.txt", 14848514);
        root.add_file("c.dat", 8504156);
        let d = root.add_directory("d");

        d.add_file("j", 4060174);
        d.add_file("d.log", 8033020);
        d.add_file("d.exe", 5626152);
        d.add_file("k", 7214296);

        println!("total size: ", root.total_size());

        assert_eq!(solve(Part::One, &TEST_INPUT), 95437);
    }

    #[test]
    fn part1() {
        assert_eq!(solve(Part::One, &TEST_INPUT), 95437);
    }

    #[test]
    fn part2() {
        assert_eq!(solve(Part::Two, &TEST_INPUT), 42);
    }
}

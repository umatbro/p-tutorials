use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::{Rc, Weak}, collections::vec_deque::Iter,
};

#[derive(Debug)]
pub struct File {
    name: String,
    size: u64,
    parent: RefCell<Weak<Directory>>,
}

#[derive(Debug, Default)]
pub struct Directory {
    name: String,
    files: RefCell<Vec<Rc<File>>>,
    directories: RefCell<Vec<Rc<Directory>>>,
    parent: RefCell<Weak<Directory>>,
}

impl Directory {
    pub fn add_file(parent: &Rc<Self>, name: String, size: u64) -> Rc<File> {
        let parent_w = Rc::downgrade(parent);

        let file = File {
            name,
            size,
            parent: RefCell::new(parent_w),
        };
        let file = Rc::new(file);
        let mut files = parent.as_ref().files.borrow_mut();
        files.push(Rc::clone(&file));

        Rc::clone(&file)
    }

    pub fn add_directory(parent: &Rc<Self>, name: String) -> Rc<Directory> {
        let parent_w = Rc::downgrade(parent);

        let dir = Directory {
            name,
            parent: RefCell::new(parent_w),
            ..Default::default()
        };
        let dir = Rc::new(dir);
        let mut dirs = parent.as_ref().directories.borrow_mut();
        dirs.push(Rc::clone(&dir));

        Rc::clone(&dir)
    }

    pub fn calc_size(&self) -> u64 {
        let mut sum = 0;
        for file in self.files.borrow().iter() {
            sum += file.size;
        }
        for dir in self.directories.borrow().iter() {
            sum += dir.calc_size()
        }
        sum
    }

    pub fn get_subdirs(&self) -> Vec<Rc<Directory>> {
        let mut result = Vec::new();
        if self.directories.borrow().is_empty() {
            return Vec::new();
        }
        for dir in self.directories.borrow().iter() {
            result.push(Rc::clone(dir));
            result.extend(dir.get_subdirs());
        }
        result
    }
}

#[derive(Debug)]
pub struct Tree {
    root: RefCell<Rc<Directory>>,
    current_dir: RefCell<Rc<Directory>>,
}

#[derive(Debug, PartialEq)]
pub enum CdError {
    NotFound,
    NoParentForRoot,
    Conflict,
}

impl Tree {
    pub fn new() -> Self {
        let root = Rc::new(Directory {
            name: String::from(""),
            files: RefCell::new(Vec::new()),
            directories: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        });
        Self {
            root: RefCell::new(Rc::clone(&root)),
            current_dir: RefCell::new(Rc::clone(&root)),
        }
    }

    pub fn current_dir(&self) -> Rc<Directory> {
        Rc::clone(&self.current_dir.borrow())
    }

    pub fn change_dir(&mut self, name: &str) -> Result<Rc<Directory>, CdError> {
        let current_dir = Rc::clone(&self.current_dir.borrow());
        if name == ".." {
            let parent = current_dir.parent.borrow().upgrade();
            match parent {
                Some(dir) => {
                    self.current_dir = RefCell::new(Rc::clone(&dir));
                    return Ok(Rc::clone(&dir));
                }
                None => return Err(CdError::NoParentForRoot),
            }
        } else if name == "/"  {
            let root = Rc::clone(&self.root.borrow());
            self.current_dir = RefCell::new(Rc::clone(&root));
            return Ok(Rc::clone(&root));
        }
        let dirs_binding = current_dir.directories.borrow();

        let search_result: Vec<&Rc<Directory>> = dirs_binding
            .iter()
            .filter(|dir| dir.name == String::from(name))
            .collect();

        if search_result.len() > 1 {
            return Err(CdError::Conflict);
        }
        let searched_subdir = search_result.first();

        match searched_subdir {
            Some(dir) => {
                self.current_dir = RefCell::new(Rc::clone(&dir));
                Ok(Rc::clone(dir))
            }
            None => Err(CdError::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::tree::CdError;

    use super::Directory;
    use super::Tree;

    #[test]
    fn test_create_tree() {
        let tree = Tree::new();
        assert_eq!(tree.root.borrow().name, String::from(""));
    }

    #[test]
    fn test_add_file() {
        let tree = Tree::new();
        let file = Directory::add_file(&tree.root.borrow(), String::from("Domi"), 10);

        let root = tree.root.borrow();
        let root_files = root.files.borrow();
        let in_tree = root_files.get(0).unwrap();
        assert!(Rc::ptr_eq(&file, in_tree));
        assert_eq!(Rc::strong_count(&file), 2);

        let parent_from_file = in_tree.parent.borrow().upgrade().unwrap();
        let parent_from_tree = tree.root.borrow();
        assert!(Rc::ptr_eq(&parent_from_file, &parent_from_tree));
    }

    #[test]
    fn test_add_directory() {
        let tree = Tree::new();
        let file = Directory::add_directory(&tree.root.borrow(), String::from("Domi"));

        let root = tree.root.borrow();
        let root_files = root.directories.borrow();
        let in_tree = root_files.get(0).unwrap();
        assert!(Rc::ptr_eq(&file, in_tree));
        assert_eq!(Rc::strong_count(&file), 2);

        let parent_from_file = in_tree.parent.borrow().upgrade().unwrap();
        let parent_from_tree = tree.root.borrow();
        assert!(Rc::ptr_eq(&parent_from_file, &parent_from_tree));
    }

    fn setup_example_tree() -> Tree {
        let tree = Tree::new();
        {
            let root = tree.root.borrow();

            let a = Directory::add_directory(&root, String::from("a"));
            let _b = Directory::add_file(&root, String::from("b.txt"), 14848514);
            let _c = Directory::add_file(&root, String::from("c.dat"), 8504156);
            let d = Directory::add_directory(&root, String::from("d"));

            let e = Directory::add_directory(&a, String::from("e"));
            let _f = Directory::add_file(&a, String::from("f"), 29116);
            let _g = Directory::add_file(&a, String::from("g"), 2557);
            let _h = Directory::add_file(&a, String::from("h.lst"), 62596);

            let _i = Directory::add_file(&e, String::from("i"), 584);

            let _j = Directory::add_file(&d, String::from("j"), 4060174);
            let _k = Directory::add_file(&d, String::from("k"), 7214296);
            let _d_log = Directory::add_file(&d, String::from("d.log"), 8033020);
            let _d_ext = Directory::add_file(&d, String::from("d.ext"), 5626152);
        }

        tree
    }

    #[test]
    fn test_calc_size() {
        let mut tree = setup_example_tree();
        let a = tree.change_dir("a").unwrap();
        let e = tree.change_dir("e").unwrap();
        tree.change_dir("..").unwrap();
        let root = tree.change_dir("..").unwrap();
        let d = tree.change_dir("d").unwrap();

        assert_eq!(e.calc_size(), 584);
        assert_eq!(d.calc_size(), 24933642);
        assert_eq!(a.calc_size(), 94853);
        assert_eq!(root.calc_size(), 48381165);
    }

    #[test]
    fn test_change_dir() {
        let mut tree = setup_example_tree();
        assert_eq!(tree.current_dir().name, String::from(""));

        let a_dir = tree.change_dir("a").unwrap();
        assert_eq!(a_dir.name, String::from("a"));
        let cderr = tree.change_dir("non-exist").err().unwrap();
        assert_eq!(cderr, CdError::NotFound);
        assert_eq!(tree.current_dir().name, String::from("a"));

        let e_dir = tree.change_dir("e").unwrap();
        assert_eq!(e_dir.name, String::from("e"));
        assert_eq!(tree.current_dir().name, String::from("e"));
        let go_up_dir = tree.change_dir("..").unwrap();
        assert_eq!(go_up_dir.name, String::from("a"));
        assert_eq!(tree.current_dir().name, String::from("a"));

        let go_up_root = tree.change_dir("..").unwrap();
        assert_eq!(go_up_root.name, String::from(""));
        assert_eq!(tree.current_dir().name, String::from(""));

        let no_parent_for_root = tree.change_dir("..").err().unwrap();
        assert_eq!(no_parent_for_root, CdError::NoParentForRoot);
        assert_eq!(tree.current_dir().name, String::from(""));
        assert!(tree.current_dir().parent.borrow().upgrade().is_none());
    }

    #[test]
    fn test_change_to_root() {
        let mut tree = setup_example_tree();
        tree.change_dir("a").unwrap();
        tree.change_dir("e").unwrap();

        {
            let current_dir = tree.current_dir();
            let parent = current_dir.parent.borrow().upgrade();
            assert!(parent.is_some());
        }

        tree.change_dir("/").unwrap();
        {
            let current_dir = tree.current_dir();
            let parent = current_dir.parent.borrow().upgrade();
            assert!(parent.is_none());
        }
    }

    #[test]
    fn test_get_subdirs() {
        let tree = setup_example_tree();
        let flat_dirs = tree.current_dir().get_subdirs();

        let names: Vec<String> =  flat_dirs.iter().map(|d| String::from(&d.name)).collect();
        assert_eq!(names, vec!["a".to_string(), "e".to_string(), "d".to_string()]);
    }
}

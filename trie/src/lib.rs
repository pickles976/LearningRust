use std::collections::HashMap;

pub struct Node {
    is_end: bool,
    children: HashMap<char, Box<Node>>,
}

impl Node {

    pub fn new(is_end: bool) -> Node {
        Node {
            children: HashMap::new(),
            is_end,
        }
    }

    pub fn check_value(self, c: char) -> bool {
        self.children.contains_key(&c)
    }

    pub fn insert_value(&mut self, c: char, is_end: bool){
        self.children.insert(c, Box::new(Node::new(is_end)));
    }

    pub fn print_all(&self, s: String) {
        for k in self.children.keys() {
            let mut new_s = s.clone();
            new_s.push(k.clone());
            self.children.get(k).unwrap().print_all(new_s);
        }

        if self.is_end {
            println!("{}", s);
        }
    }
}

pub struct Trie {
    root: Node,
}

impl Trie {

    pub fn new() -> Trie {
        Trie {
            root: Node::new(false),
        }
    }

    pub fn insert(&mut self, s: String){
        let mut current_node = &mut self.root;
        let char_list: Vec<char> = s.chars().collect();
        let mut last_match = 0;

        for (i, value) in char_list.iter().enumerate(){
            if current_node.children.contains_key(value){
                current_node = current_node.children.get_mut(value).unwrap()
            }else{
                last_match = i;
                break;
            }
            last_match = i + 1
        }

        if last_match == char_list.len() {
            current_node.is_end = true;
        } else {
            for new_counter in last_match..char_list.len() {
                current_node.insert_value(char_list[new_counter], false);
                current_node = current_node
                    .children
                    .get_mut(&char_list[new_counter])
                    .unwrap();
            }
            current_node.is_end = true;
        }

    }

    pub fn find(&mut self, s: String) -> bool {
        let mut current_node = &mut self.root;
        let char_list: Vec<char> = s.chars().collect();

        for (i, value) in char_list.iter().enumerate() {
            if !current_node.children.contains_key(value) {
                return false;
            } else {
                current_node = current_node
                    .children
                    .get_mut(value)
                    .unwrap();
            }
        }
        return true;
    }

    pub fn print_all(self) {

        self.root.print_all("".to_string());

    }

}
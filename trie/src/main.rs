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
}

struct Trie {
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
                // println!(
                //     "Inserting {}",
                //     char_list[new_counter],
                // );
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

}

fn main() {

    let mut trie = Trie::new();

    trie.insert("AMBULANCE".to_string());
    trie.insert("AMBULATORY".to_string());
    trie.insert("APPALACHIA".to_string());
    trie.insert("ALLSAICE-LORRAINE".to_string());
    trie.insert("AMPERSAND".to_string());

    println!("Is AMBULANCE in the tree? {}", trie.find("AMBULANCE".to_string()));
    println!("Is SUS in the tree? {}", trie.find("SUS".to_string()));

}
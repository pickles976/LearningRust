#[cfg(test)]
mod tests {
    use std::collections::FxHashMap;

    use bytebuffer::ByteBuffer;

    use crate::{count_characters, get_leaves, get_heap, encode_contents, rebuild_tree};

    #[test]
    fn test_counting() {
        let map = count_characters(&"mmmmaao".to_string());
        assert_eq!(4, map.get(&'m').unwrap().clone());
        assert_eq!(2, map.get(&'a').unwrap().clone());
        assert_eq!(1, map.get(&'o').unwrap().clone());
        assert!(map.get(&'f').is_none());
    }

    #[test]
    fn test_tree() {

        /*

            Test that tree shape is:

                None, 7
                /     \
            m, 4    None, 3
                    /       \
                  a, 2      o, 1
        */

        // basic tree
        let contents = "mmmmaao".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let root = &heap.peek().unwrap().0;

        assert!(root.c.is_none());

        let left = root.l.as_ref().unwrap();
        assert_eq!('m' , left.c.unwrap());

        let right = root.r.as_ref().unwrap();
        assert!(right.c.is_none());

        let right_left = right.l.as_ref().unwrap();
        assert_eq!('a', right_left.c.unwrap());

        let right_right = right.r.as_ref().unwrap();
        assert_eq!('o', right_right.c.unwrap());

        /*

                    Test that tree shape is:

                        None, 11
                        /       \
                None, 7       None, 4
                /    \      /       \
            None, 4   ' '  ?, 2      ?, 2
            /     \
          ?, 2    ?, 2
        */

        // more complex tree
        let contents = "ll tt oo aa".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let root = &heap.peek().unwrap().0;

        assert!(root.c.is_none());

        let left = root.l.as_ref().unwrap();
        assert!(left.c.is_none());

        let right = root.r.as_ref().unwrap();
        assert!(right.c.is_none());

        let right_left = right.l.as_ref().unwrap();
        assert!(right_left.c.is_some());

        let right_right = right.r.as_ref().unwrap();
        assert!(right_right.c.is_some());

        let left_right = left.r.as_ref().unwrap();
        assert_eq!(' ', left_right.c.unwrap());

        let left_left = left.l.as_ref().unwrap();
        assert!(left_left.c.is_none());

        let left_left_left = left_left.l.as_ref().unwrap();
        assert!(left_left_left.c.is_some());

        let left_left_right = left_left.r.as_ref().unwrap();
        assert!(left_left_right.c.is_some());

    }

    #[test]
    fn test_tree_representation() {

        let contents = "mmmmaao".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let mut tree_string = "".to_string();

        let _= &heap.peek().unwrap().0.save_tree(&mut tree_string);

        assert_eq!("00111", tree_string);

        let contents = "ll tt oo aa".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let mut tree_string = "".to_string();

        let _= &heap.peek().unwrap().0.save_tree(&mut tree_string);

        assert_eq!("001101011", tree_string);

    }

    #[test]
    fn test_codes() {

        // easy encoding
        let contents = "mmmmaao".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let mut codes: FxHashMap<char, String> = FxHashMap::default();
        heap.peek().unwrap().0.get_codes("".to_string(), &mut codes);

        assert_eq!("0", codes.get(&'m').unwrap());
        assert_eq!("10", codes.get(&'a').unwrap());
        assert_eq!("11", codes.get(&'o').unwrap());
        assert!(codes.get(&'f').is_none());

        // harder encoding
        let contents = "ll aa oo tt".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);

        let mut codes: FxHashMap<char, String> = FxHashMap::default();
        heap.peek().unwrap().0.get_codes("".to_string(), &mut codes);

        assert_eq!("01", codes.get(&' ').unwrap());
        assert!(codes.get(&'f').is_none());

    }

    #[test]
    fn test_encoding() {

        // easy tree
        let contents = "mmmmaao".to_string();
        let map = count_characters(&contents);
        let leaves = get_leaves(map);
        let heap = get_heap(leaves);
        let mut characters: String = "".to_string();
        heap.peek().unwrap().0.get_character_order(&mut characters);
        let mut codes: FxHashMap<char, String> = FxHashMap::new();
        heap.peek().unwrap().0.get_codes("".to_string(), &mut codes);
        let mut binary_string = "".to_string();
        heap.peek().unwrap().0.save_tree(&mut binary_string);

        let mut byte_buffer: ByteBuffer = encode_contents(&binary_string, &characters, &contents, codes);

        let mut byte;

        // 4 (length) + 1 (tree) + 4 * 3 (chars) + 2 (data) = 19 bytes
        println!("{:?}", byte_buffer.to_bytes());
        assert_eq!(19, byte_buffer.len());

        // tree 00111 -> 7 LE
        byte = byte_buffer.read_u32();

        assert_eq!(7, byte);
        
        // tree 00111 -> 7 LE
        let mut byte_8 = byte_buffer.read_u8();

        assert_eq!(56, byte_8);

        // chars 'o', 'a', 'm'
        byte = byte_buffer.read_u32();

        assert_eq!('o' as u32, byte);

        byte = byte_buffer.read_u32();

        assert_eq!('a' as u32, byte);

        byte = byte_buffer.read_u32();

        assert_eq!('m' as u32, byte);

        // text 0000 1010 11 -> 10, 192
        byte_8 = byte_buffer.read_u8();

        assert_eq!(10, byte_8);

        byte_8 = byte_buffer.read_u8();

        assert_eq!(192, byte_8);

    }

    #[test]
    fn test_tree_from_bytes() {

        /*

            Test that tree shape is:

                None, 7
                /     \
            m, 4    None, 3
                    /       \
                  a, 2      o, 1
        */

        let bytes = [0, 0, 0, 7, 56, 0, 0, 0, 'o' as u8, 0, 0, 0, 'a' as u8, 0, 0, 0, 'm' as u8];

        let mut byte_buffer = ByteBuffer::new();
        byte_buffer.write_bytes(&bytes);
        let size = byte_buffer.read_u32();
        let mut root = rebuild_tree(&mut byte_buffer);
        root = root.populate_tree(&mut byte_buffer);

        assert!(root.c.is_none());

        let left = root.l.as_ref().unwrap();
        assert_eq!('m' , left.c.unwrap());

        let right = root.r.as_ref().unwrap();
        assert!(right.c.is_none());

        let right_left = right.l.as_ref().unwrap();
        assert_eq!('a', right_left.c.unwrap());

        let right_right = right.r.as_ref().unwrap();
        assert_eq!('o', right_right.c.unwrap());

    }

}
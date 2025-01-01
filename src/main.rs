use blockchain_from_scratch::{MerkleTree, Sha256Hashable};

#[derive(Debug)]
pub struct DataObject {
    pub data: Vec<u8>,
}

impl AsRef<[u8]> for DataObject {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

fn main() {
    println!("Blockchain from scratch in Rust!");

    let leaf_a = DataObject { data: "1 - Hello, world!".to_string().into_bytes() };
    let leaf_b = DataObject { data: "2 - Hello, Rust!".to_string().into_bytes() };
    let leaf_c = DataObject { data: "3 - Hello, Blockchain!".to_string().into_bytes() };
    let leaf_d = DataObject { data: "4 - Hello, Blockchain!".to_string().into_bytes() };

    println!("{:?}", leaf_a.hash_to_hex());
    println!("{:?}", leaf_b.hash_to_hex());
    println!("{:?}", leaf_c.hash_to_hex());
    println!("{:?}", leaf_d.hash_to_hex());
    
    let mut tree = MerkleTree::<DataObject>::new(None);
    tree.add_leaf(leaf_a);
    tree.add_leaf(leaf_b);
    tree.add_leaf(leaf_c);
    tree.add_leaf(leaf_d);

    println!("{:?}", tree);
}


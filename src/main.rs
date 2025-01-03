use blockchain_from_scratch::MerkleTree;

#[derive(Debug, Clone)]
pub struct DataObject {
    pub data: Vec<u8>,
}

impl AsRef<[u8]> for DataObject {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

fn main() {
    // println!("Blockchain from scratch in Rust!");

    // let leaf_a = DataObject { data: "first item".to_string().into_bytes() };
    // let leaf_b = DataObject { data: "second item".to_string().into_bytes() };
    // let leaf_c = DataObject { data: "third item".to_string().into_bytes() };
    // let leaf_d = DataObject { data: "fourth item".to_string().into_bytes() };

    // println!("Leaf A: {:?}", leaf_a.hash_to_hex());
    // println!("Leaf B: {:?}", leaf_b.hash_to_hex());
    // println!("Leaf C: {:?}", leaf_c.hash_to_hex());
    // println!("Leaf D: {:?}", leaf_d.hash_to_hex());

    // let mut tree = MerkleTree::<DataObject>::new(None);
    // tree.add_leaf(leaf_a);
    // tree.add_leaf(leaf_b);
    // tree.add_leaf(leaf_c);
    // tree.add_leaf(leaf_d);

    // println!("{:?}", hex::encode(tree.calculate_root().unwrap()));

    // let proof = tree.get_proof(1).unwrap();
    // println!("{:?}", proof);
}


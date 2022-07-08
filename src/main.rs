use zeroize::Zeroize;

#[derive(Clone, Copy)]
struct Scalar {
    bytes: [u8; 32],
}

impl Zeroize for Scalar {
    fn zeroize(&mut self) {
        panic!("zeroize called");
    }
}

fn main() {
    let scalar = Scalar {
        bytes: [1; 32],
    };
    let vec_of_scalars: Vec<Scalar> = vec![scalar];
    drop(vec_of_scalars);
    panic!("zeroize never called");
}

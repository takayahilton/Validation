#[derive(Debug)]
enum Validation<A, E> {
    Success(A),
    Failure(Vec<E>)
}

impl <A, E> Validation<A, E> {
    fn map<B, F: FnOnce(A) -> B>(self, func: F) -> Validation<B, E> {
        match self {
            Validation::Success(a) => Validation::Success(func(a)),
            Validation::Failure(v) => Validation::Failure(v)
        }
    }
}



fn main() {
    let suc = Validation::Success::<u32, String>(1).map(|i| i * 2);
    println!("{:?}", suc);
}

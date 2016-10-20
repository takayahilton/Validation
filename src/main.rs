#[derive(Debug)]
enum Validation<A, E> {
    Success(A),
    Failure(Vec<E>)
}

impl<A, E: Clone> Validation<A, E> {

    fn map<B, F: FnOnce(A) -> B>(self, func: F) -> Validation<B, E> {
        match self {
            Validation::Success(a) => Validation::Success(func(a)),
            Validation::Failure(v) => Validation::Failure(v)
        }
    }

    fn ap<B, F: FnOnce(A) -> B>(self, func: Validation<F, E>) -> Validation<B, E> {
        match (self, func) {
            (Validation::Success(a), Validation::Success(f)) => Validation::Success(f(a)),
            (Validation::Failure(v), Validation::Success(_)) => Validation::Failure(v),
            (Validation::Success(_), Validation::Failure(v)) => Validation::Failure(v),
            (Validation::Failure(v1), Validation::Failure(v2)) => Validation::Failure(v1.iter().chain(v2.iter()).map(|e| e.clone()).collect()),
        }
    }

    fn flat_map<B, F: FnOnce(A) -> Validation<B, E>>(self, func: F) -> Validation<B, E> {
        match self {
            Validation::Success(a) => func(a),
            Validation::Failure(v) => Validation::Failure(v)
        }
    }
}



fn main() {
    use Validation::*;

    let suc = Failure::<u32, &str>(vec!["fail1"])
        .ap(Failure::<u32, &str>(vec!["fail2"])
            .ap(Success(move |i| move |j| i + j)
            )
        );
    println!("{:?}", suc);
}

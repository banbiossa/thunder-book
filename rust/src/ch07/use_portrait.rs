#[portrait::make]
trait CoreState {
    fn is_done(&self) -> bool;
}

struct Data {
    turn: usize,
}

impl CoreState for Data {
    fn is_done(&self) -> bool {
        self.turn > 0
    }
}

struct Wrap {
    inner: Data,
}

#[portrait::fill(portrait::delegate(Data; self.inner))]
impl CoreState for Wrap {}

fn use_trait<T: CoreState>(state: &T) {
    println!("is done? {}", state.is_done());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_trait() {
        let data = Data { turn: 0 };
        use_trait(&data);

        let wrapped = Wrap { inner: data };
        use_trait(&wrapped);
    }
}

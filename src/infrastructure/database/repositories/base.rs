pub trait Repo<Executor> {
    fn new(executor: Executor) -> Self;
}

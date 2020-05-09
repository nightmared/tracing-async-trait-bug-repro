use async_trait::async_trait;
use tracing::instrument;

fn main() {
    println!("Hello, world!");
}



#[async_trait]
pub trait Foo
where
    Self: std::marker::Send,
{
    async fn foo(&self) -> ();
}

#[derive(Debug)]
struct FooImpl;

#[async_trait]
impl Foo for FooImpl {
    #[instrument]
    async fn foo(&self) {}
}



#[instrument]
async fn top_level() {
}

use tracing::instrument;

fn main() {
    println!("Hello, world!");
}



#[tonic::async_trait]
pub trait Foo
where
    Self: std::marker::Send,
{
    async fn foo(&self) -> ();
}

struct FooImpl;

#[tonic::async_trait]
impl Foo for FooImpl {
    #[instrument]
    async fn foo(&self) {}
}



#[instrument]
async fn top_level() {
}

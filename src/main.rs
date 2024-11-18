use rsh::Shell;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Shell::new().run().await
}

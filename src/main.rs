use shell_rs::Shell;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Shell::new().run().await
}

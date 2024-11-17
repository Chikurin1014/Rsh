use shell_rs::Shell;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    Shell::new().run().await
}

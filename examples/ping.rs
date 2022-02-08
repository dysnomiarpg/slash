use slash::{Context, Filter};

#[tokio::main]
async fn main() {
    let ping = slash::command("ping")
        .description("Test the bot's latency.")
        .map(|context: Context| context.reply("pong!"));
}

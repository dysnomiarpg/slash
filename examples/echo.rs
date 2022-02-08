use slash::{Context, Filter};

#[tokio::main]
async fn main() {
    let echo = slash::command("echo")
        .and(slash::arg::string("text", "The text to echo."))
        .map(|context: Context, arg: String| {});
}

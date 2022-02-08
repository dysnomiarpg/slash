# slash

Slash is a Discord interaction library for Rust.

> 🚧 **Slash is currently in development and is not suited for production.** Please do not file issues or report bugs related to the production environment. 🚧

## Examples

```rs
async fn main() -> Result<(), Box<dyn Error>> {
	let ping = 
		slash::command("ping")
			.description("Ping the bot")
			.map(|ctx: &Context| async {
				ctx.reply("Pong!").await?
			});
	
	let echo = 
		slash::command("echo")
			.description("Echo a message")
			.and(slash::arg("message"))
			map(|ctx: &Context, message: String| {
				ctx.reply(&message).await?
			});

	let cmds = ping.or(echo);

	slash::serve(cmds).await
}
```

## License

You are able to use Slash under two different licenses:

-   Free license
-   Commercial license

The free license is a modified version of the [MIT license](https://opensource.org/licenses/MIT), with the conditions that:

-   you are an individual, or a non-profit or not-for-profit organisation
-   generate no revenue through the usage of Slash
-   your bot is in less than 1000 guilds

A commercial license is also available for larger organisations, or bots that generate revenue.

More information can be found in the [LICENSE](LICENSE.md) file.

# slashlib

Slashlib is a Discord interaction library for Rust.

> 🚧 **Slashlib is currently in development and is not suited for production.** Please do not file issues or report bugs related to the production environment. 🚧

## Examples

```rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	// create the "ping" command
	let ping = ChatCommandBuilder::new()
		.set_name("ping")
		.set_description("Replies with 'pong!'")
		.on_execute(|ctx| async {
			ctx.reply("Pong!");
			Ok(())
		})
		.build();
	// create the web client and start listening
	let client = slashlib::web::Client::new()
		.add_command(ping)
		.listen()
		.await?
}
```

## License

You are able to use slashlib under two different licenses:

-   Free license
-   Commercial license

The free license is a modified version of the [MIT license](https://opensource.org/licenses/MIT), with the conditions that:

-   you are an individual, or a non-profit or not-for-profit organisation
-   generate no revenue through the usage of slashlib
-   your bot is in less than 1000 guilds

A commercial license is also available for larger organisations, or bots that generate revenue.

More information can be found in the [LICENSE](LICENSE.md) file.

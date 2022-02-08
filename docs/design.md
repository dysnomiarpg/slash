# Design

## Filter System

Similar to [`warp`](https://github.com/seanmonstar/warp)'s filter system, slashlib features a filter system to extract data, and run pre-conditions on commands.

```rs
let my_data = vec![];

let ping =
	slash::command("ping")
		.and(|| my_data)
		.and(slash::guild(""))
		.and(slash::permission::has_role("123897192837"))
		.and()
		.map(|ctx: &slash::Context| async {
			ctx.reply("pong!").await;
		})

let commands = ping.or(pong);
```

### Filter::and

Produces a new filter that requires both this filter and the target filter to match.

### Filter::or

Produces a new filter that requires either this filter or the target filter to match.

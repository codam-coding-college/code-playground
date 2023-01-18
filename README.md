# Code executor
An extendible remote code execution webserver.

- [ ] Use the config file
- [ ] Re-write in Rust
- [ ] Dockerize and use Kuberenetes for load balancing
- [ ] Find all the possible exploits
- [ ] Allow to ban keywords (say to forbid using certain imports)
- [ ] Make sure that containers are completely disconnected from the internet.

## API

A clients incoming request looks as follows:
```json
{
	"lang": "c",
	"flags": "-Wextra -Werror -Wall",
	"code": "<Some code>"
}
```

A servers outgoing response should look as follows:
```json
{
	"result": "Request received!",
	"error": null
}
```

OR

```json
{
	"result": null,
	"error": "This code is god awful!"
}
```
# Code executor
An extendible remote code execution webserver.

- [x] Use the config file
- [x] Re-write in Rust
- [ ] Dockerize and use Kuberenetes for load balancing
- [ ] Find all the possible exploits
- [ ] Allow to ban keywords (say to forbid using certain imports)
- [ ] Make sure that containers are completely disconnected from the internet.

## Docker

```bash
docker build -t code-playground .
docker run -d --restart unless-stopped -p 4242:4242 --name code-playground code-playground
docker stop code-playground
docker rm code-playground
```

## API

A clients incoming request looks as follows:
```json
{
	"langauge": "c",
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

# Media Manager

⚠🛠 **Under Development** 🛠⚠ _See [DISCLAIMER](#disclaimer)._

This is a simple microservice-oriented project for downloading
bittorrent files, possibly modifying them with `ffmpeg`, and 
making them available via HTML5 video player.

| Feature | Tool |
|---------|------|
| **UI**         | [Leptos](https://leptos.dev/) |
| **API**        | [Gin](https://gin-gonic.com/) |
| **BitTorrent** | [transmission](https://transmissionbt.com/) packaged by [linuxserver.io](https://www.linuxserver.io/) |
| **File Uploads** | [tusd](https://github.com/tus/tusd) |

_Probably [MiniO](https://min.io/) soon for better storage management..._

## Disclaimer

I just made a tool. You are solely responsible for what you do with it.
I make absolutely NO WARRANTY that this software is safe to use or fit
any purpose. Please see the [LICENSE](./LICENSE.md) for more detailed
information regarding use and redistribution of the software.

## Building and Running

Everything is containerized, so `podman-compose up --build` should
do the trick. I haven't tested it with `docker-compose`.

`nginx` binds to `localhost:8080` and serves both the UI static files
and acts as the api gateway.

## Troubleshooting

If you're using, `podman-compose` on ubuntu-22, there is a
[known bug](https://bugs.launchpad.net/ubuntu/+source/libpod/+bug/2024394)
where containers are created with the wrong CNI version. Fortunately
Alvin Ng has put forward a [workaround](https://bugs.launchpad.net/ubuntu/+source/libpod/+bug/2024394/comments/16).

## Contributing

Contributions are welcome! Open a PR and all that.

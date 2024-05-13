
# Media Manager

⚠🛠 **Under Development** 🛠⚠ 

_I threw this together in a weekend. Don't expect any miracles. See [DISCLAIMER](#disclaimer)._

This is a simple microservice-oriented project for downloading
bittorrent files, possibly modifying them with `ffmpeg`, and 
making them available via HTML5 video player.

It is really just a web UI written using the [Leptos framework](https://leptos.dev/),
a web service written using the [gin framework](https://gin-gonic.com/),
and off-the-shelf [transmission](https://transmissionbt.com/) packaged by
[linuxserver.io](https://www.linuxserver.io/).

## Disclaimer

I just made a tool. You are solely responsible for what you do with it.
I make absolutely NO WARRANTY that this software is safe to use or fit
any purpose. Please see the [LICENSE](./LICENSE.md) for more detailed
information regarding use and redistribution of the software.

## Building the UI with `trunk`

Use [`trunk`](https://trunkrs.dev/) to build the UI:

```sh
cd ui && trunk build --release
```

This will create `ui/dist`, which will be served by nginx.

## Running the Containers

Either `podman-compose` or `docker-compose` should work, although I
haven't tested `docker-compose`.

```
podman-compose up
```

You can then access the UI at `http://localhost:8080` and the
transmission UI at `http://localhost:8080/transmission`.

## Troubleshooting

If you're using, `podman-compose` on ubuntu-22, there is a
[known bug](https://bugs.launchpad.net/ubuntu/+source/libpod/+bug/2024394)
where containers are created with the wrong CNI version. Fortunately
Alvin Ng has created a [workaround](https://bugs.launchpad.net/ubuntu/+source/libpod/+bug/2024394/comments/16).

## Contributing

Contributions are welcome! Open a PR and all that.

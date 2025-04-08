# AniList Push Notifications

A tool to receive AniList notifications via Pushover.

## Command Line Arguments

| Short | Long              | Description                      |
|-------|-------------------|----------------------------------|
| `-a`  | `--app-token`     | Set the Pushover App Token       |
| `-u`  | `--user-token`    | Set the Pushover User Token      |
| `-t`  | `--anilist-token` | Set the AniList Token            |
| `-r`  | `--reset`         | Reset the latest notification ID |

First, [install Rust](https://www.rust-lang.org/tools/install), then install using
```sh
cargo install anilist-push
```

Run using `anilist-push`.

For example:
```bash
anilist-push -a YOUR_PUSHOVER_APP_TOKEN -u YOUR_PUSHOVER_USER_TOKEN -t YOUR_ANILIST_TOKEN
```

Arguments:

Read the [AniList docs](https://docs.anilist.co/guide/auth/) on how to generate a user token.
Read the [Pushover API documentation](https://pushover.net/api) on how to get an app and user token/key.
# mpd-slack-status

> This is a Rust rewrite of <https://github.com/hawx/mpd-slack-status>. Since
> I'm still learning Rust and therefore I have less faith in this code you may
> want to use the original instead of this.

Change your Slack status to the currenly playing track in MPD.

## getting started...

This doesn't integrate as an official API user so is slightly more annoying to setup.

The easiest way to grab the data needed is to open the team chat with the web
client and open a JS console. Then grab the following,

```js
TS.model.api_token
// this is for --api-token

TS.model.api_url
// this needs the site name prepending, then pass to --api-url
// usually just /api/ so you need to pass https://whatever-team.slack.com/api/

TS.boot_data.version_uid
// this is for --version-uid
```

Then you should be able to run it like so,

```sh
$ git clone https://github.com/hawx/mpd-slack-status-rs
$ cd mpd-slack-status-rs
$ cargo install
$ mpd-slack-status \
     --api-token 'xoxs-something-something' \
     --api-url 'https://my-team.slack.com/api/' \
     --version-uid 'aebcaaeaabebebabcba' \
     --default-emoji ":question:" \
     --default-text "I'm doing something"
```

I have no idea how long these tokens last for, yet.

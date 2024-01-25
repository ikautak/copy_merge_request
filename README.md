# create a copy of the GitLab merge request with a changed target branch.

1. Get access token
   https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html
1. copy .env-sample and change `ACCESS_TOKEN`
1. edit `config.json`
   set source merge request id and target branch list.
1. `cargo run -- -c config.json`

# copy GitLab merge request, changing the target branches.

1. Get access token
   https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html
1. copy .env-sample and change `ACCESS_TOKEN`
1. edit `config.json`
   set source merge request id and target branch list.
1. `cargo run -- -c config.json`

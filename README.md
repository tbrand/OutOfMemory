# OutOfMemory

## Concept and Features

OutOfMemory is a <i>developers community</i> which user can post questions and answers inspired by [StackOverflow](https://stackoverflow.com/).
It's assumued to be used internally, like in company or school.

OutOfMemory is mainly written in [**Rust**](https://github.com/rust-lang/rust) (backend) and [**Vue.js**](https://github.com/vuejs/vue) (frontend).
I believe this repository could be a **good example** for those who will build a web app in that combination.

Expecially the backend uses **awesome crates** which you should use for developing APIs.
- [seanmonstar/warp](https://github.com/seanmonstar/warp)
- [diesel-rs/diesel](https://github.com/diesel-rs/diesel)
- [sfackler/r2d2](https://github.com/sfackler/r2d2)
- [serde-rs/serde](https://github.com/serde-rs/serde) ([serde-rs/json](https://github.com/serde-rs/json), [dtolnay/serde-yaml](https://github.com/dtolnay/serde-yaml), [nox/serde_urlencoded](https://github.com/nox/serde_urlencoded))
- [rust-lang-nursery/failure](https://github.com/rust-lang-nursery/failure)
- [seanmonstar/reqwest](https://github.com/seanmonstar/reqwest)

<i>This repository is still under developing and many of functions are not working! (Pagination, sorting and so on.)</i>

## Getting Start! (Build and Launch)

There is a only 3 steps to start running OutOfMemory! You need `docker` and `docker-compose` command in your environment.

### 1. Create an OAuth app on [GithHub](https://github.com/settings/developers) and save it as config.yaml.
```yaml
github:
  client_id: "YOUR_CLIENT_ID"
  client_secret: "YOUR_CLIENT_SECRET"
  oauth_access_token: "https://github.com/login/oauth/access_token"
  oauth_authorize: "https://github.com/login/oauth/authorize"
  api_endpoint: "https://api.github.com"
```

There is a skeleton file at [oom-backend/config.sample.yaml](/oom-backend/config.sample.yaml). You don't have to replace `oauth_access_token`, `oauth_authorize` and `api_endpoint` if you use github.com.

### 2. Modify the [oom-backend/docker-compose.sample.yml](/oom-backend/docker-compose.sample.yml).

Copy the skeleton file and rename it as `docker-compose.yml`.
```yaml
cp docker-compose.sample.yml docker-compose.yml
```

Modify the `./path_to/your/config` to the correct path which the config created at step 1 exists on.

### 3. Start running!

```bash
# It takes a long time!
docker-compose build
docker-compose up
```

#### Note
1. If you **don't** running it on your localhost, please commit `.env.docker-compose.local` on `oom-frontend/.`. Otherwise the frontend try to connect to `http://localhost:8080` as written in `oom-frontend/.env.docker-compose`.
```
VUE_APP_API_ENDPOINT=http://your_host:8080
```

## Gallery

### Create a Question
<kbd><img width="1680" alt="outofmemory01" src="https://user-images.githubusercontent.com/3483230/60758892-a4dc2300-a057-11e9-9a4f-ec3960a70d40.png"></kbd>

### Question and Answers
<kbd><img width="1680" alt="outofmemory02" src="https://user-images.githubusercontent.com/3483230/60758934-1f0ca780-a058-11e9-9eb0-fdc63912b063.png"></kbd>

### User Profile
<kbd><img width="1680" alt="outofmemory04" src="https://user-images.githubusercontent.com/3483230/60758936-23d15b80-a058-11e9-8ad9-4e4b06452ec1.png"></kbd>

### Tags
<kbd><img width="1680" alt="outofmemory03" src="https://user-images.githubusercontent.com/3483230/60758935-216f0180-a058-11e9-9cf1-ee461467571e.png"></kbd>

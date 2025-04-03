# Data Pipeline App

## Setup
### Requirements
You will need either Docker or [Finch](https://runfinch.com/docs/getting-started/installation/) installed. If you use Docker, just replace `finch` with `docker` in the below commands.


To run both the controlplane API and Postgres database using docker:
```sh
$ finch compose -f docker/docker-compose.dev.yml build
$ finch compose -f docker/docker-compose.dev.yml up -d

# later
$ finch compose -f docker/docker-compose.dev.yml down
```

The above is useful for example when you are developing the frontend and don't need to make changes to the controlplane API. However, if you are making changes to the API, it's usually easier to only start the Postgres container and run the API locally using `cargo run`:
```sh
# only runs DB container
$ finch compose -f docker/docker-compose.dev.yml up -d db

# run the control plane API
$ cargo run

# or to automatically reload binary when code changes
$ cargo install cargo-watch
$ cargo watch -x run
```


### Usage
Once the controlplane API and Database are up and running, you can make queries against the API. I recommend installing [HTTPie](https://httpie.io/cli).

```sh
# create a user
$ http :3000/users email=johndoe@example.com password=12345
{
    "id": "bb355a6f-a7c2-4d14-ba59-bbc433e2f4f5"
}

# try to get user without credentials
$ http :3000/users/bb355a6f-a7c2-4d14-ba59-bbc433e2f4f5 
{
    "error": "Missing credentials"
}

# login as user
$ http :3000/auth/login email=johndoe@example.com password=12345
{
    "access_token": "exampleiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImV3aW5ncnlhbjk4QGdtYWlsLmNvbSIsImV4cCI6MTc0MzY2ODM2OX0.KIDR3vFw6Jar-7K9dU_xq5u4SjemW6DFtNWocpuv2os",
    "token_type": "Bearer"
}

# Use JWT token to make request and get user
$ http :3000/users/bb355a6f-a7c2-4d14-ba59-bbc433e2f4f5 Authorization:"Bearer exampleJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImV3aW5ncnlhbjk4QGdtYWlsLmNvbSIsImV4cCI6MTc0MzY2ODM2OX0.KIDR3vFw6Jar-7K9dU_xq5u4SjemW6DFtNWocpuv2os"
{
    "created_at": "2025-04-03T02:19:00.728586Z",
    "email": "johndoe@example.com",
    "id": "bb355a6f-a7c2-4d14-ba59-bbc433e2f4f5"
}

```
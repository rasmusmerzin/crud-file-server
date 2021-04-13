<h1>
  <img src="./logo.svg" />
  <br />
  crud-file-server
  <br />
  <a href="https://gitlab.com/rasmusmerzin/crud-file-server/-/commits/master">
    <img alt="build" src="https://img.shields.io/gitlab/pipeline/rasmusmerzin/crud-file-server/master" />
  </a>
  <a href="https://crates.io/crates/crud-file-server">
    <img alt="license" src="https://img.shields.io/crates/l/crud-file-server" />
  </a>
  <a href="https://crates.io/crates/crud-file-server">
    <img alt="crates" src="https://img.shields.io/crates/v/crud-file-server?label=crates.io" />
  </a>
  <a href="https://hub.docker.com/r/rasmusmerzin/crud-file-server">
    <img alt="docker" src="https://img.shields.io/docker/v/rasmusmerzin/crud-file-server?label=docker&sort=semver" />
  </a>
</h1>

A simple HTTP file server with POST, GET, PUT, PATCH and DELETE methods.

## Environment Variables

- `DIRECTORY` – directory path to write to and read files from  
  default: `DIRECTORY=content`
- `SERVER_ADDR` – server socket address  
  default: `SERVER_ADDR=0.0.0.0:8000`

## Example

Start

```
$ cargo run
```

Create (upload)

```
$ curl localhost:8000 -d Hello
3718021f-1c23-4dcb-9a90-6d1a74709744
```

Read (download)

```
$ curl localhost:8000/3718021f-1c23-4dcb-9a90-6d1a74709744
Hello
```

Update (overwrite)

```
$ curl localhost:8000/3718021f-1c23-4dcb-9a90-6d1a74709744 -X put -d Hey
3
$ curl localhost:8000/3718021f-1c23-4dcb-9a90-6d1a74709744
Hey
```

Update (append)

```
$ curl localhost:8000/3718021f-1c23-4dcb-9a90-6d1a74709744 -X patch -d ', World!'
11
$ curl localhost:8000/3718021f-1c23-4dcb-9a90-6d1a74709744
Hey, World!
```

Delete

```
$ curl localhost:8000/3718021f-1c23-4dcb-9a90-6d1a74709744 -X delete
```

## Docker Image

```
$ docker run -p 8000:8000 rasmusmerzin/crud-file-server
```

## Cargo Crate

```
$ cargo install crud-file-server
```

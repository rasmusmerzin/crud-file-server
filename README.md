<h1>
  crud-file-server
  <br />
  <a href="https://gitlab.com/rasmusmerzin/crud-file-server/-/commits/master">
    <img alt="build" src="https://img.shields.io/gitlab/pipeline/rasmusmerzin/crud-file-server/master" />
  </a>
  <a href="https://crates.io/crates/crud-file-server">
    <img alt="license" src="https://img.shields.io/crates/l/crud-file-server" />
  </a>
  <a href="https://crates.io/crates/crud-file-server">
    <img alt="version" src="https://img.shields.io/crates/v/crud-file-server" />
  </a>
  <a href="https://hub.docker.com/repository/docker/rasmusmerzin/crud-file-server">
    <img alt="size" src="https://img.shields.io/docker/image-size/rasmusmerzin/crud-file-server" />
  </a>
</h1>

A simple HTTP file server with PUT, GET and DELETE methods.

## Environment Variables

- `DIRECTORY` – directory path to write to and read files from  
  default: `DIRECTORY=content` (relative)
- `SERVER_ADDR` – server socket address  
  default: `SERVER_ADDR=0.0.0.0:8000`

## Example

Start

```
$ cargo run
```

Create (upload)

```
$ curl -T <file_path> localhost:8000
3718021f-1c23-4dcb-9a90-6d1a74709744
```

Read (download)

```
$ curl localhost:8000/3718021f-1c23-4dcb-9a90-6d1a74709744
<file>
```

Delete

```
$ curl -X delete localhost:8000/3718021f-1c23-4dcb-9a90-6d1a74709744
```

## Docker Image

```
$ docker run -p 8000:8000 rasmusmerzin/crud-file-server
```

## Cargo Crate

```
$ cargo install crud-file-server
```

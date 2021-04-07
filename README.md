# crud-file-server

[![pipeline status](https://gitlab.com/rasmusmerzin/crud-file-server/badges/master/pipeline.svg)](https://gitlab.com/rasmusmerzin/crud-file-server/-/commits/master)

A simple HTTP fileserver with PUT, GET and DELETE methods.

## Environment Variables

- `DIRECTORY` – directory path to write to and read files from  
  default: `DIRECTORY=content` (relative)
- `SERVER_ADDR` – server socket address  
  default: `SERVER_ADDR=0.0.0.0:8000`

## Example

### Start

```
$ cargo run
```

### Create

```
$ curl -T <file_path> localhost:8000
3718021f-1c23-4dcb-9a90-6d1a74709744
```

### Read

```
$ curl localhost:8000/3718021f-1c23-4dcb-9a90-6d1a74709744
<file>
```

### Delete

```
$ curl -X delete localhost:8000/3718021f-1c23-4dcb-9a90-6d1a74709744
```

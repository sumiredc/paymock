# migration

- https://diesel.rs/

## How to Migration

### create file
```sh
diesel migration generate {name}
```

### migrate
```sh
diesel migration run
```

### redo
```sh
# one
diesel migration redo

# all
diesel migration redo --all
```

### revert
```sh
# one
diesel migration revert

## all
diesel migration revert --all
```

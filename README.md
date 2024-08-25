# common-rs
common library crates

##### Publish to crates.io (Steps)
<strike>

```language
1. git commit with corresponding changes
2. create tag: git tag -a releases/0.0.1 -m "version 0.0.1"
3. tag push with commits e.g. (git push origin "branches" --tags)
```
</strike>

##### Deprecated services
<strike>

```language
novax-services will mark deprecated for future commits
The corresponding services will be reside on its own package
Such as http and grpc will become two projects instead of just one contains both
```
</strike>


We'll be leverage the commit message to indicated whether the package will publish o crates.io
following is an example to publish novax-log package

1. Commit the change as ususal.
2. Increate the vesion number in Cargo.toml of the project.
3. Within the commit message highlight the inclusion of "PUBLISH LOG ....."

The workflow will be trigger once git push has invoked.

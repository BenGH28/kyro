# WIP: ☧ kyro - the Bible on the command line

- Currently a work in progress while I finish off school. THERE WILL BE BUGS.
- The listed commands below are planned to be implemented

# What is implemented?
- [x] search for passage within a chapter
- [ ] search for multi-chapter passage
- [ ] read command
- [ ] today command
- [x] help
- [ ] list-bibles

## Commands

-   Help

```
kyro --help
```

-   List available Bibles

```sh
kyro --list-bibles
```

-   Read/search a passage of scripture

```sh
kyro read John 3:16

# start from the beginning of the book
kyro read John

# search for a range of verses
kyro search John 3:16-18
```

-   Verse of the day

```
kyro today
```

# WIP: ☧ kyro - the Bible on the command line

- Currently, a work in progress while I look for work. THERE WILL BE BUGS.
- The listed commands below are planned to be implemented

# What is implemented?
- [x] Search for passage within a chapter
- [x] Search for multi-chapter passage
- ~~[ ] Read command~~ - re-evaluating including this
    - was originally included so that you could read in a pager but not sure if this is practical anymore
- [ ] Today command
- [x] Help
- [ ] support multiple languages - just English so far but I'd like to add more
- [ ] Listing Bibles - just NET so far but with more languages there could be more
- [ ] Format output text for easier reading - checkout [ ttf_word_wrap ]( https://docs.rs/ttf_word_wrap/0.5.0/ttf_word_wrap/index.html )

## Commands

-   Help

```list
kyro help
```

-   List available Bibles

```sh
kyro bibles
```

- Search for a passage of scripture

```sh
# search for a range of verses
kyro search John 3:16-18
```

-   Verse of the day

```
kyro today
```

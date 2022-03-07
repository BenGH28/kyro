# WIP: ☧ kyro - the Bible on the command line

- I'm using this project as an excuse to learn Rust so bear with me...THERE WILL BE BUGS.
- The listed commands below are planned to be implemented


## Dependencies
If you're on Linux then you should already have `less` installed if not go ahead and install it:

### macOS
```sh
brew install less
```
### Ubuntu
```sh
sudo apt install less
```
### Arch
```sh
sudo pacman -S less
```

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

- Read a book of the Bible with `less`
```sh
# read a book of the Bible starting from the beginning
kyro read John

# or read a book of the Bible starting at a specific chapter
kyro read John 3
```

- Verse of the day

```
kyro today
```

## To-do
- [x] Help
- [x] Search for passage within a chapter
- [x] Search for multi-chapter passage
- [x] Read command
- [x] Today command
- [ ] Format output text for easier reading - checkout [ ttf_word_wrap ]( https://docs.rs/ttf_word_wrap/0.5.0/ttf_word_wrap/index.html )
- [ ] support multiple languages - just English so far but I'd like to add more
- [ ] Listing Bibles - just NET so far but with more languages there could be more

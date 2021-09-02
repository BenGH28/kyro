# ☧ kyro - the Bible on the command line

## Potential Bible Sources

-   API's
    -   [API.Bible](https://scripture.api.bible/)
        -   free for non-commercial use but I'd need to use my API key for everyone maybe unless everyone makes their own account with the site and gets their own key (potenial for abuse of service?).
        -   has multiple bibles available
    -   [ESV API](https://api.esv.org/)
        -   ESV only and need to do a bunch of stuff with copyright most likely
-   Non-API
    -   [Gratis Bible](https://github.com/gratis-bible/bible)
        -   many many languages availble each with multiple bible support
        -   can store things locally

## Selected Bible Sources

-   **Gratis Bible**
    -   offers much ease for me
    -   I can use it as a submodule in the project and reference that maybe

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

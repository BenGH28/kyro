# Configuration
I want the user to be able to  configure the app.  What shall we configure?

1. Bible
2. Language

There will be a config file where we can set this `$XDG_CONFIG_HOME/.config/kyro/config.toml`

```toml
#this should be the default config
[Language]
english

[Bible]
asv
```

Kyro will read the config file and get the appropriate bible xml file from [gratis-bible](https://github.com/gratis-bible/bible).
If the user wants to try another language or Bible version they can do so from the command line.
```sh
# use french and use the Ostervald Bible
$ kyro --lang=french --bible=ost jean 3:16
```

Languages might be tricky because I only understand English but I think I can hand in the book argument and parse the Bible xml from that.
Kyro will download the bible and place it somewhere on your computer probably `$XDG_DATA_HOME` which is likely
`$HOME/.local/share`


# CommandLine

The help list
```
kyro -h
	Usage: kyro <options> [command] [args]


	Options:
		-h --help 					print this help message
		-b --bible=[bible version]  		set your prefered Bible version
		-l --lang=[LANGUAGE]    		set your prefered language
		-ll --language-list 			list all the languages supported
		-bl --bible-list 			list all the Bibles for your selected language

	Commands:
		read [args]  				read a portion of the Bible in a buffer
		search [args] 				search for section of the Bible and print
							it to the terminal
		today 					Get a verse of the day printed to the screen


	Args:
		<Book>  				Start this book from 1:1
		<Book> <Chapter> 			Start this book at this chapter from
							verse 1 in a buffer
		<Book> <Chapter>:<Verse>  		Start this book from the chapter and verse
		<Book> <Chapter>:<Verse>-<Verse>  	Get this section of the book
```


#  The Bible
We will store the specified Bible locally on the user's computer under the preferred language.
If the user specifies a different language on the commandline or changes the config then we check that
we don't already have it in on the computer and we download it if necessary.


# Todo
- [ ] Make English work first
	- [ ] Get a Bible from gratis-bible
		- [ ] store it in `$XDG_DATA_HOME`
	- [ ] parse the Bible xml
	- [ ] print the Bible to the terminal
	- [ ] show the Bible in a buffer like `man` or `more` (make the buffer uneditable).





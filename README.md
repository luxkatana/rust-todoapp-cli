# A boring todoapp in rust


## What does this do?

It's a todoapp written in rust, but I made a cli-application to interact with the todo-app 

## How it works


It uses SQLite3 to save the todo data

##Running the application

> It's just as easy as stealing a candy from a baby

<ol>
<li>Install the rustup toolchain from https://rustup.rs</li>
<li>Clone this repository</li>
</oL>

There are 2 versions you can build from this project:
<ol>
<li>The debug version </li>
<li>The release version </li>
</ol>

## To build and run the debug version

### To build the debug version of the todo-app, run the following command
```
cargo b
```

If you ran this command for the first time, you'll see a new directory called "target", inside there is a directory called "debug"
### To run the program
As mentioned above, go inside the target/debug folder and the executable is **cli-todo** (or **cli-todo.exe** if you're on windows)


##To build and run the release version

### Building the release version
To build the release version of the cli-todo project, run the following command

```
cargo b --release
```

The key dfiference between the debug version and the release version is that the release version is optimized and faster for production usage.

Just like the debug version, you'll find a target folder and inside there you'll find a release folder.

### Running the release version

As mentioned above, go into the target/release folder. There you'll find an executable called **cli-todo** (or **cli-todo** if you're on windows) and voila! That is the executable :tada:



## Reason why I made this

As many rustaceans mentioned has rust a difficult learning curve, so I read the documentation (the awesome rust book) twice to rehearse the things in rust.

And I wanted to learn how to create a CLI in rust :p

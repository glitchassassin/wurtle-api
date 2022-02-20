# wurtle-api

A sample Rust API for a word game. This is half an experiment in Rust and half an experiment in solving Wordle puzzles. To test solving algorithms, I needed an API to emulate a Wordle game, so I wrote one.

PRs welcome to fix issues, follow best practices, or add features.

## Usage

You can pull the latest version of the container from Docker Hub and run it:

```
docker run -P 8000:8000 glitchassassin/wurtle-api
```

Then make a POST request to http://localhost:8000/guess with a JSON body like the following:

```
{
    "guess": "train"
}
```

The response will include your results, as well as a "word" property, which is a unique key for the random word you've been given:

```
{
    "result": [
        "CORRECT",
        "CORRECT",
        "WRONG",
        "WRONG",
        "WRONG"
    ],
    "word": 4869,
    "win": false
}
```

For future guesses against the same word, just include it in your request body:

```
{
    "guess": "truck",
    "word": 4869
}
```
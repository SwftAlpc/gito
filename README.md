# Gito

A command line tool that opens the remote URL of a Git repository.

## Feature
- You can open the GitHub page for the current branch by simply typing the command.
- You can use the `-p` option to create a pull request for a specified branch.
  - You can use the `-b` option to specify the base branch. (default main)

## Usage
```
// Open the GitHub page for the current branch
gito
```

```
// Open the pull request page with the specified branch as the base branch
gito -p -b develop
```

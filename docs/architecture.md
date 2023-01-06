# Architecture

This file contains an architecture's description of `mm` application.

## High-level perspective

The app consists of several components:

- repositories component: it manages all the tasks, that can be done with a notes repository.
  This component actually is an interface to git;
- data components: it is responsible for storing any application's data, such as configurations
  and repositories;
- tree controller: it provides an interface to notes, folders and repositories;
- editor.

## Components responsibility

This section describes interfaces of each component.

### Repositories

`mm` has one main repository, that is called `mm_main_local`. It exists by default (actually
created at its first use). User is allowed to create a custom repository and bind it with
an external git-based source control service (such as GitHub).

Repositories can be used simultaneously. One can store some quick notes in the main `mm`'s
repository and some important notes in the separate repository, that will be shared across
devices via git server.

Repositories component provides a way to:

- open or create a local repository;
- link a local repository to a remote one (except `mm_main_local`);
- clone a remote repository;
- add a folder to a repository;
- add a note to a repository;
- modify a note and commit changes;
- restore a signle note to some previous state;
- fetch remote changes;
- push changes to remote;
- enumerate all repostories;
- enumerate all folders in a repository;
- enumerate all notes in a folder;
- enumerate all notes in a repository.

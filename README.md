# map_reduce_benchmark

Implementation of a Map Reduce problem in Rust.

## Prerequisites

Using this repository will require an installation of [Python](https://www.python.org/downloads/) and [Rust](https://www.rust-lang.org/tools/install).

## The Problem

Find the number of shared "friend" connections between individuals in a social network 

*i.e. A has 23 friends in common with B.*

## Example data

### Data format

We will create a dataset for a set of users `F` represented by integers `1...N` where each user has a random set of friends `f` within `F`. For this example each user will have 100 friends.

Real world social networks exhibit "clustering" of connections (all users do not have equal probabilities of being friends with one another). To simulate this, we will draw friend connections from the 100 nearest users to a given user. i.e. for user `f_i` the set of possible friends will be a set `f` of integers between `f_i - 50...f_i + 50`. We will do something different for the first and last groups of 50 users.

### Data structure

Data will be in user-specific `.csv` files where each file is named for the user and contains a list of connections to that user.

### Data generation

To create the example dataset - run `datagen.py`.

```shell
python datagen.py
```

## The Solution

### Map

Map reduce definition: *the map step performs filtering and sorting of input data.*

In this example the map function will take a user `.csv` file and create a key, value pair:

  *key: sorted list of the user and one friend within f.*
  *value: vector of friends f.*

For each f_i in f.

*Note: friendships are an undirected graph (if A -> B, B -> A). Sorting the key will make keys the same for the same pairs of users: for A -> B, key = (A, B) and for B -> A, key = (A, B).*

*Question: Can this be done in parallel? - yes this can. The combination step, don't think so.*

### Combination

The output of the map function will be grouped by key so that a key now contains a vector of friend vectors for a pair of friends.

### Reduce

Map reduce definition: *the reduce step performs an aggregation operation.*

The reduce function will return the original key and the length of the intersection of the elements of the vectors of friends.

### Usage

Compile and run the solution from the respository root with:

```shell
rustc rust/main.rs && ./main
```

## Sources:

[The original problem.](https://stackoverflow.com/questions/12375761/good-mapreduce-examples)
[Map Reduce overview.](https://en.wikipedia.org/wiki/MapReduce)

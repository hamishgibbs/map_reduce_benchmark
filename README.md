# map_reduce_benchmark

Implementation of a Map Reduce problem in Rust.

## Prerequisites

## The Problem

Find the number of shared "friend" connections between individuals in a social network (i.e. A has 23 friends in common with B).

## Example data

**Data format**

We will create a dataset for a set of users F represented by integers 1...N where each user has a random set of friends f within F. For this example each user can have a random selection of between 0 and 100 friends.

Real world social networks exhibit "clustering" of connections (all users do not have equal probabilities of being friends with one another). To simulate this, we will draw friend connections from the 1000 nearest users to a given user. i.e. for user f_i the set of possible friends will be a set f of integers between f_i - 500...f_i + 500. We will do something different for the first and last groups of 500 users.

**Data structure**

Data will be in user-specific `.csv` files where each file is named for the user and contains a list of connections to that user.

**Data generation**

To calculate the size of an example dataset and (optionally) create one - run `datagen.py`.

**Bug**

There is an issue with the creation of the data - nobody has any shared friends with anyone else. I.e. with current construction of example data - there are no pairs of shared friendships (A is in f of B and B is in f of A. This is a requirement of the training data)

So - need to think up a new way to do that. Hmmm....

## The Solution

**Map**

*The map step performs filtering and sorting of input data.*

In this example the map function will take a user `.csv` file and create a key, value pair:

  *key: sorted list of the user and one friend within f.*
  *value: vector of friends f.*

For each f_i in f.

*This is an undirected graph (if A -> B, B -> A). Sorting the key will make keys the same for the same pairs of users: for A -> B, key = (A, B) and for B -> A, key = (A, B).*

*Question: Won't this greatly increase the amount of memory needed to store the data? Perhaps there is no way to avoid this to do the comparison.*
*Question: Can this be done in parallel? - yes this can. The combination step, I don't think so.*

**Grouping**

The output of the map function will be grouped by key so that a key now contains a vector of friend vectors.

**Reduce**

*The reduce steps performs an aggregation operation.*

The reduce function will return the original key and the intersection of the elements of the value vector.

## Development

1. Write the datagen step in `datagen.py` X
  1. Calculate how much memory will be taken up for some N users X
  2. Create a data directory if it doesn't exist. If it does, delete all contents before writing to it. X
2. Handle the case where n is with in +- 500 of the end of the list. X
3. Write out a csv file per user (1.csv, 2.csv) with no headers. X

1. Try it out in Rust and anticipate a major headache. But remember that the major headache is the point. Keep commenting and referring to the spec.

1. Try it out in Go and anticipate a major headache. But remember that the major headache is the point. Keep commenting and referring to the spec.

## Benchmarks

## Sources:

[The original problem.](https://stackoverflow.com/questions/12375761/good-mapreduce-examples)
[Map Reduce overview.]()

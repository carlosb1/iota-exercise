# Ledger Statistics

# Instructions
- Code must be written from scratch.
- Choose a language of your choice. You may work in whatever programming language you
are comfortable with, using any libraries you want (as long as we can compile and run your
code).
- You may create a repo for the solution under your github account, but it’s also fine to submit
the result as a zip.
- Deadline is 2 weeks.
- Don’t hesitate to ask clarification questions if needed.

# Objective
You're provided with a list of transactions as input. They represent the nodes of a directed
(acyclic) graph D and have been reduced to:

- Their unique identifier (id)
- Two transactions that they reference (left & right)
- A timestamp
Your task is to write a small program that parses this list in memory and returns statistics for
D. Using this, you can calculate a number of statistics:

- Average depth of D per node (assume that the graph has a root node r (the node
with id=1) then the depth of node t is the length of the shortest r-t path in D)
- Average number of nodes per depth (not including depth 0)
- Average number of in-references. i.e. indegree, per node
- Be creative. Find at least one other statistic that you think would be interesting to have.



## Inputs & Outputs
Input for this problem should be a "database" in the form of a plain text file, with a structure as
follows:


- Line 1: N, the number of nodes in the database. Assume N < 10,000
- Lines 2 through N+1: the node data, where each node consists of the IDs of its left and
right parents, and a timestamp
- The id of a node in the database is its line number, e.g. the node in the second
line has id=2.
- Node id=1 denotes the root of D, i.e. the unique origin of all transactions. It is
always present in any version of D, but it is not stored in the database.

## Database template
```
1 N # integer, number of nodes, N < 10000
2 L R T # integers describing a node, L and R = Left and Right parent node
IDs, T =
Timestamp
3 ... #
4 ...
5 ...
```
The program is expected to be run and output to the console as follows:
```
$ ./ledgerstats database.txt
> AVG DAG DEPTH: ???
> AVG TXS PER DEPTH: ???
> AVG REF: ???
> <YOUR STAT>: ???
```

## Example

database.txt:
```
5
1 1 0
1 2 02 2 1
3 3 2
3 4 3
```
Running and output:
```
$ ./ledgerstats database.txt
> AVG DAG DEPTH: 1.33
> AVG TXS PER DEPTH: 2.5
> AVG REF: 1.667
```

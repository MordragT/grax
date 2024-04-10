# Graph library

TODO

split tests into unit tests using mocking insde grax algorithms
integration tests for algorithms in grax_impl
benches of integration tests for algorithms in grax_impl

- improve fixed size attribute maps by default vec which is essentially a stable (like vec option) vec returning the default value for non existent values
- implement dot file format (graphviz)
- make filter views uncoupled from VisitMaps and allow custom filter functions (use case dfs/bfs marker functions)
- move union_find, route, parents, distances to grax-algorithms
- implement connected graph via GraphFacade and custom NodeStorage
- implemented way to get multiple nodes or edges from multiple indices, or way to run closures over multiple edges/nodes by index
  - could also enable simd graph implementation where visitmap is essentially a mask of all edges and then iter_adjacent can directly filter with few instructions edges out
- implement way to create approximate graphs (e.g. creating a new graph with 1/2 node size by keeping only nodes which have atleas threshold neighbors)
- implemement cdcl:
  - nodes have constraints assoicated(requirements) with them. Edges have concrete constraints they resolve associated with them
  - 1. build initial dependency graph (implication graph)
  - 2. traverse graph when problem occurs do cdcl magic and backtrack (add constraints based on the problem that occured to the initial graph and try again)

- implement Dir Graph which uses a directory for its node and edge storage
- make traits faillable
## Benchmarks

- initial

```
test tests::breadth_search_connected_components_graph1                ... bench:         374 ns/iter (+/- 12)
test tests::breadth_search_connected_components_graph2                ... bench:      42,322 ns/iter (+/- 1,332)
test tests::breadth_search_connected_components_graph3                ... bench:      37,617 ns/iter (+/- 2,308)
test tests::breadth_search_connected_components_graph_ganz_ganz_gross ... bench: 298,468,871 ns/iter (+/- 46,563,870)
test tests::breadth_search_connected_components_graph_ganz_gross      ... bench:  88,745,909 ns/iter (+/- 9,736,944)
test tests::breadth_search_connected_components_graph_gross           ... bench:  19,259,770 ns/iter (+/- 4,279,775)
test tests::depth_search_connected_components_graph1                  ... bench:         320 ns/iter (+/- 58)
test tests::depth_search_connected_components_graph2                  ... bench:      29,571 ns/iter (+/- 4,112)
test tests::depth_search_connected_components_graph3                  ... bench:      26,585 ns/iter (+/- 1,632)
test tests::depth_search_connected_components_graph_ganz_ganz_gross   ... bench: 289,822,032 ns/iter (+/- 44,371,347)
test tests::depth_search_connected_components_graph_ganz_gross        ... bench:  99,734,340 ns/iter (+/- 11,296,568)
test tests::depth_search_connected_components_graph_gross             ... bench:  18,209,196 ns/iter (+/- 1,905,625)

```

## Edmonds Karp

initial

```
running 6 tests
test algorithms::edmonds_karp::test::edmonds_karp_fluss2_adj_list        ... bench:       2,185 ns/iter (+/- 212)
test algorithms::edmonds_karp::test::edmonds_karp_fluss2_adj_mat         ... bench:       2,384 ns/iter (+/- 80)
test algorithms::edmonds_karp::test::edmonds_karp_fluss_adj_list         ... bench:       1,018 ns/iter (+/- 63)
test algorithms::edmonds_karp::test::edmonds_karp_fluss_adj_mat          ... bench:       1,030 ns/iter (+/- 124)
test algorithms::edmonds_karp::test::edmonds_karp_g_1_2_adj_list         ... bench:     599,931 ns/iter (+/- 205,676)
test algorithms::edmonds_karp::test::edmonds_karp_g_1_2_adj_mat          ... bench:   1,737,554 ns/iter (+/- 73,951)

```

new

```
running 6 tests
test edmonds_karp::test::edmonds_karp_fluss2_adj_list        ... bench:       6,824 ns/iter (+/- 229)
test edmonds_karp::test::edmonds_karp_fluss2_adj_mat         ... bench:       7,318 ns/iter (+/- 23)
test edmonds_karp::test::edmonds_karp_fluss_adj_list         ... bench:       3,106 ns/iter (+/- 36)
test edmonds_karp::test::edmonds_karp_fluss_adj_mat          ... bench:       2,664 ns/iter (+/- 22)
test edmonds_karp::test::edmonds_karp_g_1_2_adj_list         ... bench:   1,035,364 ns/iter (+/- 32,227)
test edmonds_karp::test::edmonds_karp_g_1_2_adj_mat          ... bench:  14,706,582 ns/iter (+/- 458,575)

```


## double tree

initial

```
test double_tree::test::double_tree_k_10_adj_list            ... bench:      14,157 ns/iter (+/- 452)
test double_tree::test::double_tree_k_10_adj_mat             ... bench:      17,408 ns/iter (+/- 82)
test double_tree::test::double_tree_k_10e_adj_list           ... bench:      13,746 ns/iter (+/- 475)
test double_tree::test::double_tree_k_10e_adj_mat            ... bench:      17,617 ns/iter (+/- 180)
test double_tree::test::double_tree_k_12_adj_list            ... bench:      21,566 ns/iter (+/- 213)
test double_tree::test::double_tree_k_12_adj_mat             ... bench:      29,113 ns/iter (+/- 150)
test double_tree::test::double_tree_k_12e_adj_list           ... bench:      21,564 ns/iter (+/- 265)
test double_tree::test::double_tree_k_12e_adj_mat            ... bench:      29,304 ns/iter (+/- 125)
```

new

```
test double_tree::test::double_tree_k_10_adj_list            ... bench:       8,602 ns/iter (+/- 368)
test double_tree::test::double_tree_k_10_adj_mat             ... bench:      10,437 ns/iter (+/- 698)
test double_tree::test::double_tree_k_10e_adj_list           ... bench:       8,344 ns/iter (+/- 738)
test double_tree::test::double_tree_k_10e_adj_mat            ... bench:      10,020 ns/iter (+/- 289)
test double_tree::test::double_tree_k_12_adj_list            ... bench:      12,677 ns/iter (+/- 152)
test double_tree::test::double_tree_k_12_adj_mat             ... bench:      14,803 ns/iter (+/- 1,477)
test double_tree::test::double_tree_k_12e_adj_list           ... bench:      12,224 ns/iter (+/- 305)
test double_tree::test::double_tree_k_12e_adj_mat            ... bench:      14,998 ns/iter (+/- 199)


```

## brute force

initial

```
test brute_force::test::brute_force_k_10_adj_list            ... bench: 2,559,025,262 ns/iter (+/- 72,692,736)
```

parallel

```
test brute_force::test::brute_force_k_10_adj_list            ... bench: 1,481,942,610 ns/iter (+/- 12,224,827)
```

## kruskal

initial

```
test kruskal::test::kruskal_graph_10_20_adj_list             ... bench:  10,886,441 ns/iter (+/- 1,027,674)
test kruskal::test::kruskal_graph_10_20_adj_mat              ... bench: 222,182,749 ns/iter (+/- 3,188,631)
test kruskal::test::kruskal_graph_1_200_adj_list             ... bench: 144,564,947 ns/iter (+/- 1,980,924)
test kruskal::test::kruskal_graph_1_20_adj_list              ... bench:   8,271,378 ns/iter (+/- 89,344)
test kruskal::test::kruskal_graph_1_20_adj_mat               ... bench: 232,524,293 ns/iter (+/- 6,149,869)
test kruskal::test::kruskal_graph_1_2_adj_list               ... bench:     789,944 ns/iter (+/- 15,189)
test kruskal::test::kruskal_graph_1_2_adj_mat                ... bench:   2,954,700 ns/iter (+/- 67,072)
```

new

```
test kruskal::test::kruskal_graph_10_200_adj_list            ... bench:  75,621,255 ns/iter (+/- 12,085,628)
test kruskal::test::kruskal_graph_10_200_adj_mat             ... bench:  64,980,060 ns/iter (+/- 2,366,164)
test kruskal::test::kruskal_graph_10_20_adj_list             ... bench:   5,740,724 ns/iter (+/- 486,666)
test kruskal::test::kruskal_graph_10_20_adj_mat              ... bench:   4,954,061 ns/iter (+/- 48,389)
test kruskal::test::kruskal_graph_1_200_adj_list             ... bench:  64,029,416 ns/iter (+/- 5,166,913)
test kruskal::test::kruskal_graph_1_200_adj_mat              ... bench:  64,890,534 ns/iter (+/- 2,638,657)
test kruskal::test::kruskal_graph_1_20_adj_list              ... bench:   4,311,643 ns/iter (+/- 630,989)
test kruskal::test::kruskal_graph_1_20_adj_mat               ... bench:   4,453,875 ns/iter (+/- 246,663)
test kruskal::test::kruskal_graph_1_2_adj_list               ... bench:     411,561 ns/iter (+/- 2,304)
test kruskal::test::kruskal_graph_1_2_adj_mat                ... bench:     434,446 ns/iter (+/- 8,175)
```

native cpu

```
test kruskal::test::kruskal_graph_10_200_adj_list            ... bench:  39,678,650 ns/iter (+/- 12,416,472)
test kruskal::test::kruskal_graph_10_200_csr_mat             ... bench:  37,659,044 ns/iter (+/- 12,532,907)
test kruskal::test::kruskal_graph_10_200_hash_graph          ... bench:  36,614,371 ns/iter (+/- 12,215,747)
test kruskal::test::kruskal_graph_10_20_adj_list             ... bench:   4,691,095 ns/iter (+/- 1,501,017)
test kruskal::test::kruskal_graph_10_20_csr_mat              ... bench:   4,340,062 ns/iter (+/- 1,497,099)
test kruskal::test::kruskal_graph_10_20_dense_mat            ... bench:  64,363,510 ns/iter (+/- 32,484,880)
test kruskal::test::kruskal_graph_10_20_hash_graph           ... bench:   4,768,071 ns/iter (+/- 1,785,233)
test kruskal::test::kruskal_graph_1_200_adj_list             ... bench:  38,039,580 ns/iter (+/- 11,532,992)
test kruskal::test::kruskal_graph_1_200_csr_mat              ... bench:  32,535,717 ns/iter (+/- 12,313,503)
test kruskal::test::kruskal_graph_1_200_dense_mat            ... bench:  43,528,782 ns/iter (+/- 10,176,149)
test kruskal::test::kruskal_graph_1_200_hash_graph           ... bench:  34,144,815 ns/iter (+/- 12,495,269)
test kruskal::test::kruskal_graph_1_20_adj_list              ... bench:   4,500,780 ns/iter (+/- 1,427,149)
test kruskal::test::kruskal_graph_1_20_csr_mat               ... bench:   4,379,244 ns/iter (+/- 1,442,359)
test kruskal::test::kruskal_graph_1_20_dense_mat             ... bench:   7,416,408 ns/iter (+/- 2,123,536)
test kruskal::test::kruskal_graph_1_20_hash_graph            ... bench:   4,005,795 ns/iter (+/- 1,409,197)
test kruskal::test::kruskal_graph_1_2_adj_list               ... bench:     530,282 ns/iter (+/- 267,496)
test kruskal::test::kruskal_graph_1_2_csr_mat                ... bench:     583,454 ns/iter (+/- 216,204)
test kruskal::test::kruskal_graph_1_2_dense_mat              ... bench:   1,528,706 ns/iter (+/- 717,982)
test kruskal::test::kruskal_graph_1_2_hash_graph             ... bench:     513,760 ns/iter (+/- 284,874)
```

## prim

- initial

```
test prim::test::prim_graph_100_200_adj_list                 ... bench:  38,040,007 ns/iter (+/- 1,285,612)
test prim::test::prim_graph_100_200_csr_mat                  ... bench:  37,068,365 ns/iter (+/- 2,767,727)
test prim::test::prim_graph_10_200_adj_list                  ... bench:   7,656,358 ns/iter (+/- 310,996)
test prim::test::prim_graph_10_200_csr_mat                   ... bench:   7,398,929 ns/iter (+/- 100,205)
test prim::test::prim_graph_10_20_adj_list                   ... bench:   2,231,449 ns/iter (+/- 130,493)
test prim::test::prim_graph_10_20_csr_mat                    ... bench:   2,209,662 ns/iter (+/- 10,648)
test prim::test::prim_graph_10_20_dense_mat                  ... bench:  50,265,241 ns/iter (+/- 4,137,262)
test prim::test::prim_graph_1_200_adj_list                   ... bench:   2,136,748 ns/iter (+/- 161,249)
test prim::test::prim_graph_1_20_adj_list                    ... bench:     566,977 ns/iter (+/- 44,118)
test prim::test::prim_graph_1_20_csr_mat                     ... bench:     562,697 ns/iter (+/- 15,759)
test prim::test::prim_graph_1_20_dense_mat                   ... bench:   1,725,670 ns/iter (+/- 110,742)
test prim::test::prim_graph_1_2_adj_list                     ... bench:     155,358 ns/iter (+/- 6,145)
test prim::test::prim_graph_1_2_csr_mat                      ... bench:     155,821 ns/iter (+/- 5,951)
test prim::test::prim_graph_1_2_dense_mat                    ... bench:     662,069 ns/iter (+/- 20,949)
```

native cpu

```
test prim::test::prim_graph_100_200_adj_list                 ... bench:  46,482,949 ns/iter (+/- 8,002,118)
test prim::test::prim_graph_100_200_csr_mat                  ... bench:  46,119,112 ns/iter (+/- 6,297,338)
test prim::test::prim_graph_10_200_adj_list                  ... bench:   7,805,904 ns/iter (+/- 1,107,694)
test prim::test::prim_graph_10_200_csr_mat                   ... bench:   8,202,883 ns/iter (+/- 1,291,948)
test prim::test::prim_graph_10_20_adj_list                   ... bench:   2,129,951 ns/iter (+/- 486,482)
test prim::test::prim_graph_10_20_csr_mat                    ... bench:   1,992,201 ns/iter (+/- 416,739)
test prim::test::prim_graph_10_20_dense_mat                  ... bench:  73,229,730 ns/iter (+/- 32,915,058)
test prim::test::prim_graph_1_200_adj_list                   ... bench:   2,354,946 ns/iter (+/- 387,698)
test prim::test::prim_graph_1_20_adj_list                    ... bench:     513,940 ns/iter (+/- 104,423)
test prim::test::prim_graph_1_20_csr_mat                     ... bench:     473,983 ns/iter (+/- 106,137)
test prim::test::prim_graph_1_20_dense_mat                   ... bench:   4,573,986 ns/iter (+/- 1,013,619)
test prim::test::prim_graph_1_2_adj_list                     ... bench:     145,743 ns/iter (+/- 6,204)
test prim::test::prim_graph_1_2_csr_mat                      ... bench:     148,416 ns/iter (+/- 917)
test prim::test::prim_graph_1_2_dense_mat                    ... bench:   1,125,005 ns/iter (+/- 492,778)
```
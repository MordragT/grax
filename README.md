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

### BFS

- initial

```
test tests::breadth_search_connected_components_graph1                ... bench:         374 ns/iter (+/- 12)
test tests::breadth_search_connected_components_graph2                ... bench:      42,322 ns/iter (+/- 1,332)
test tests::breadth_search_connected_components_graph3                ... bench:      37,617 ns/iter (+/- 2,308)
test tests::breadth_search_connected_components_graph_ganz_ganz_gross ... bench: 298,468,871 ns/iter (+/- 46,563,870)
test tests::breadth_search_connected_components_graph_ganz_gross      ... bench:  88,745,909 ns/iter (+/- 9,736,944)
test tests::breadth_search_connected_components_graph_gross           ... bench:  19,259,770 ns/iter (+/- 4,279,775)

```
new

```
test algorithms::bfs::test::bfs_scc_graph1_adj_list                      ... bench:         153 ns/iter (+/- 31)
test algorithms::bfs::test::bfs_scc_graph2_adj_list                      ... bench:      20,961 ns/iter (+/- 1,508)
test algorithms::bfs::test::bfs_scc_graph3_adj_list                      ... bench:      20,465 ns/iter (+/- 1,467)
test algorithms::bfs::test::bfs_scc_graph_ganz_ganz_gross_adj_list       ... bench: 177,265,464 ns/iter (+/- 19,975,956)
test algorithms::bfs::test::bfs_scc_graph_ganz_gross_adj_list            ... bench:  62,047,757 ns/iter (+/- 6,383,007)
test algorithms::bfs::test::bfs_scc_graph_gross_adj_list                 ... bench:   9,997,379 ns/iter (+/- 1,989,188)
```

### DFS

initial

```
test tests::depth_search_connected_components_graph1                  ... bench:         320 ns/iter (+/- 58)
test tests::depth_search_connected_components_graph2                  ... bench:      29,571 ns/iter (+/- 4,112)
test tests::depth_search_connected_components_graph3                  ... bench:      26,585 ns/iter (+/- 1,632)
test tests::depth_search_connected_components_graph_ganz_ganz_gross   ... bench: 289,822,032 ns/iter (+/- 44,371,347)
test tests::depth_search_connected_components_graph_ganz_gross        ... bench:  99,734,340 ns/iter (+/- 11,296,568)
test tests::depth_search_connected_components_graph_gross             ... bench:  18,209,196 ns/iter (+/- 1,905,625)
```

new

```
test algorithms::dfs::test::dfs_scc_graph1_adj_list                      ... bench:         164 ns/iter (+/- 6)
test algorithms::dfs::test::dfs_scc_graph2_adj_list                      ... bench:      18,809 ns/iter (+/- 2,880)
test algorithms::dfs::test::dfs_scc_graph3_adj_list                      ... bench:      19,552 ns/iter (+/- 1,605)
test algorithms::dfs::test::dfs_scc_graph_ganz_ganz_gross_adj_list       ... bench: 185,272,518 ns/iter (+/- 24,192,504)
test algorithms::dfs::test::dfs_scc_graph_ganz_gross_adj_list            ... bench:  66,601,702 ns/iter (+/- 621,164)
test algorithms::dfs::test::dfs_scc_graph_gross_adj_list                 ... bench:  10,111,518 ns/iter (+/- 564,903)
```

### Dijkstra

initial

```
test algorithms::dijkstra::test::dijkstra_g_1_2_di_adj_list              ... bench:      42,867 ns/iter (+/- 5,086)
test algorithms::dijkstra::test::dijkstra_g_1_2_undi_adj_list            ... bench:      29,277 ns/iter (+/- 2,484)
test algorithms::dijkstra::test::dijkstra_wege_1_di_adj_list             ... bench:         122 ns/iter (+/- 22)
test algorithms::dijkstra::test::dijkstra_wege_2_di_adj_list             ... bench:         179 ns/iter (+/- 11)
test algorithms::dijkstra::test::dijkstra_wege_3_di_adj_list             - should panic ... bench:         187 ns/iter
```

new

```
test algorithms::dijkstra::test::dijkstra_g_1_2_di_adj_list              ... bench:      46,094 ns/iter (+/- 2,482)
test algorithms::dijkstra::test::dijkstra_g_1_2_undi_adj_list            ... bench:      31,976 ns/iter (+/- 5,363)
test algorithms::dijkstra::test::dijkstra_wege_1_di_adj_list             ... bench:         144 ns/iter (+/- 30)
test algorithms::dijkstra::test::dijkstra_wege_2_di_adj_list             ... bench:         208 ns/iter (+/- 33)
test algorithms::dijkstra::test::dijkstra_wege_3_di_adj_list             - should panic ... bench:         207 ns/iter
```

### Edmonds Karp

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

new2

```
test algorithms::edmonds_karp::test::edmonds_karp_fluss2_adj_list        ... bench:       4,732 ns/iter (+/- 349)
test algorithms::edmonds_karp::test::edmonds_karp_fluss2_csr_mat         ... bench:       3,914 ns/iter (+/- 84)
test algorithms::edmonds_karp::test::edmonds_karp_fluss2_dense_mat       ... bench:       8,357 ns/iter (+/- 21)
test algorithms::edmonds_karp::test::edmonds_karp_fluss_adj_list         ... bench:       2,120 ns/iter (+/- 18)
test algorithms::edmonds_karp::test::edmonds_karp_fluss_csr_mat          ... bench:       1,981 ns/iter (+/- 16)
test algorithms::edmonds_karp::test::edmonds_karp_fluss_dense_mat        ... bench:       2,806 ns/iter (+/- 40)
test algorithms::edmonds_karp::test::edmonds_karp_g_1_2_adj_list         ... bench:     744,162 ns/iter (+/- 50,439)
test algorithms::edmonds_karp::test::edmonds_karp_g_1_2_csr_mat          ... bench:   4,371,949 ns/iter (+/- 210,311)
test algorithms::edmonds_karp::test::edmonds_karp_g_1_2_dense_mat        ... bench:  25,006,617 ns/iter (+/- 1,277,626)
```
with removal of edges (for edge: remove(edge))
```
test algorithms::edmonds_karp::test::edmonds_karp_fluss2_adj_list                      ... bench:       2,068.45 ns/iter (+/- 30.34)
test algorithms::edmonds_karp::test::edmonds_karp_fluss2_csr_mat                       ... bench:       3,143.25 ns/iter (+/- 49.43)
test algorithms::edmonds_karp::test::edmonds_karp_fluss2_dense_mat                     ... bench:       2,659.49 ns/iter (+/- 444.02)
test algorithms::edmonds_karp::test::edmonds_karp_fluss_adj_list                       ... bench:       1,103.93 ns/iter (+/- 39.66)
test algorithms::edmonds_karp::test::edmonds_karp_fluss_csr_mat                        ... bench:       1,350.02 ns/iter (+/- 224.51)
test algorithms::edmonds_karp::test::edmonds_karp_fluss_dense_mat                      ... bench:       1,160.73 ns/iter (+/- 25.22)
test algorithms::edmonds_karp::test::edmonds_karp_g_1_2_adj_list                       ... bench:     487,561.20 ns/iter (+/- 38,233.24)
test algorithms::edmonds_karp::test::edmonds_karp_g_1_2_csr_mat                        ... bench:   7,132,653.00 ns/iter (+/- 177,331.23)
test algorithms::edmonds_karp::test::edmonds_karp_g_1_2_dense_mat                      ... bench:   6,929,707.60 ns/iter (+/- 106,876.62)
```

with removal v2 (retain_edges(...))
```
test algorithms::edmonds_karp::test::edmonds_karp_fluss2_adj_list                      ... bench:       1,978.03 ns/iter (+/- 132.65)
test algorithms::edmonds_karp::test::edmonds_karp_fluss2_csr_mat                       ... bench:       2,336.36 ns/iter (+/- 39.71)
test algorithms::edmonds_karp::test::edmonds_karp_fluss2_dense_mat                     ... bench:       2,555.22 ns/iter (+/- 24.92)
test algorithms::edmonds_karp::test::edmonds_karp_fluss_adj_list                       ... bench:       1,035.72 ns/iter (+/- 18.36)
test algorithms::edmonds_karp::test::edmonds_karp_fluss_csr_mat                        ... bench:       1,031.52 ns/iter (+/- 31.36)
test algorithms::edmonds_karp::test::edmonds_karp_fluss_dense_mat                      ... bench:       1,020.38 ns/iter (+/- 11.95)
test algorithms::edmonds_karp::test::edmonds_karp_g_1_2_adj_list                       ... bench:     474,722.90 ns/iter (+/- 13,597.60)
test algorithms::edmonds_karp::test::edmonds_karp_g_1_2_csr_mat                        ... bench:   3,651,288.90 ns/iter (+/- 92,733.51)
test algorithms::edmonds_karp::test::edmonds_karp_g_1_2_dense_mat                      ... bench:   7,128,246.00 ns/iter (+/- 283,441.94)
```

### double tree

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

new2

```
test algorithms::double_tree::test::double_tree_k_10_adj_list            ... bench:       5,846 ns/iter (+/- 190)
test algorithms::double_tree::test::double_tree_k_10_csr_graph           ... bench:       5,605 ns/iter (+/- 163)
test algorithms::double_tree::test::double_tree_k_10_dense_mat           ... bench:       5,824 ns/iter (+/- 395)
test algorithms::double_tree::test::double_tree_k_10e_adj_list           ... bench:       5,914 ns/iter (+/- 153)
test algorithms::double_tree::test::double_tree_k_10e_csr_graph          ... bench:       5,648 ns/iter (+/- 106)
test algorithms::double_tree::test::double_tree_k_10e_dense_mat          ... bench:       6,026 ns/iter (+/- 216)
test algorithms::double_tree::test::double_tree_k_12_adj_list            ... bench:       8,690 ns/iter (+/- 237)
test algorithms::double_tree::test::double_tree_k_12_csr_graph           ... bench:       8,328 ns/iter (+/- 1,619)
test algorithms::double_tree::test::double_tree_k_12_dense_mat           ... bench:       8,703 ns/iter (+/- 1,525)
test algorithms::double_tree::test::double_tree_k_12e_adj_list           ... bench:       8,643 ns/iter (+/- 297)
test algorithms::double_tree::test::double_tree_k_12e_csr_graph          ... bench:       8,142 ns/iter (+/- 383)
test algorithms::double_tree::test::double_tree_k_12e_dense_mat          ... bench:       8,506 ns/iter (+/- 355)
```

new3

```
test algorithms::double_tree::test::double_tree_k_10_adj_list                          ... bench:       1,035.53 ns/iter (+/- 114.45)
test algorithms::double_tree::test::double_tree_k_10_csr_graph                         ... bench:       1,099.60 ns/iter (+/- 81.25)
test algorithms::double_tree::test::double_tree_k_10_dense_mat                         ... bench:       1,140.26 ns/iter (+/- 63.33)
test algorithms::double_tree::test::double_tree_k_10e_adj_list                         ... bench:       1,209.61 ns/iter (+/- 76.63)
test algorithms::double_tree::test::double_tree_k_10e_csr_graph                        ... bench:       1,225.16 ns/iter (+/- 19.09)
test algorithms::double_tree::test::double_tree_k_10e_dense_mat                        ... bench:       1,249.28 ns/iter (+/- 26.54)
test algorithms::double_tree::test::double_tree_k_12_adj_list                          ... bench:       1,429.42 ns/iter (+/- 28.55)
test algorithms::double_tree::test::double_tree_k_12_csr_graph                         ... bench:       1,431.47 ns/iter (+/- 27.52)
test algorithms::double_tree::test::double_tree_k_12_dense_mat                         ... bench:       1,449.07 ns/iter (+/- 58.15)
test algorithms::double_tree::test::double_tree_k_12e_adj_list                         ... bench:       1,626.03 ns/iter (+/- 37.37)
test algorithms::double_tree::test::double_tree_k_12e_csr_graph                        ... bench:       1,701.27 ns/iter (+/- 117.28)
test algorithms::double_tree::test::double_tree_k_12e_dense_mat                        ... bench:       1,801.86 ns/iter (+/- 43.01)
```

### brute force

initial

```
test brute_force::test::brute_force_k_10_adj_list            ... bench: 2,559,025,262 ns/iter (+/- 72,692,736)
```

parallel

```
test brute_force::test::brute_force_k_10_adj_list            ... bench: 1,481,942,610 ns/iter (+/- 12,224,827)
```

### kruskal

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

```
test algorithms::kruskal::test::kruskal_graph_10_200_adj_list                          ... bench:  13,225,359.80 ns/iter (+/- 1,121,458.41)
test algorithms::kruskal::test::kruskal_graph_10_200_csr_mat                           ... bench:  11,713,229.90 ns/iter (+/- 1,260,647.80)
test algorithms::kruskal::test::kruskal_graph_10_200_hash_graph                        ... bench:  12,365,528.20 ns/iter (+/- 1,210,861.34)
test algorithms::kruskal::test::kruskal_graph_10_20_adj_list                           ... bench:   1,278,225.95 ns/iter (+/- 162,393.49)
test algorithms::kruskal::test::kruskal_graph_10_20_csr_mat                            ... bench:   1,170,375.07 ns/iter (+/- 126,650.31)
test algorithms::kruskal::test::kruskal_graph_10_20_dense_mat                          ... bench:  39,900,054.50 ns/iter (+/- 2,514,839.47)
test algorithms::kruskal::test::kruskal_graph_10_20_hash_graph                         ... bench:   1,283,500.62 ns/iter (+/- 166,290.46)
test algorithms::kruskal::test::kruskal_graph_1_200_adj_list                           ... bench:  11,617,564.70 ns/iter (+/- 1,179,706.33)
test algorithms::kruskal::test::kruskal_graph_1_200_csr_mat                            ... bench:  10,420,309.50 ns/iter (+/- 895,223.22)
test algorithms::kruskal::test::kruskal_graph_1_200_dense_mat                          ... bench:  14,517,431.70 ns/iter (+/- 924,187.63)
test algorithms::kruskal::test::kruskal_graph_1_200_hash_graph                         ... bench:  11,707,997.30 ns/iter (+/- 1,163,563.51)
test algorithms::kruskal::test::kruskal_graph_1_20_adj_list                            ... bench:   1,071,045.70 ns/iter (+/- 263,192.13)
test algorithms::kruskal::test::kruskal_graph_1_20_csr_mat                             ... bench:   1,050,693.17 ns/iter (+/- 106,831.60)
test algorithms::kruskal::test::kruskal_graph_1_20_dense_mat                           ... bench:   3,327,487.90 ns/iter (+/- 194,596.88)
test algorithms::kruskal::test::kruskal_graph_1_20_hash_graph                          ... bench:   1,065,199.02 ns/iter (+/- 35,209.53)
test algorithms::kruskal::test::kruskal_graph_1_2_adj_list                             ... bench:     137,670.22 ns/iter (+/- 8,884.02)
test algorithms::kruskal::test::kruskal_graph_1_2_csr_mat                              ... bench:     126,646.11 ns/iter (+/- 11,951.51)
test algorithms::kruskal::test::kruskal_graph_1_2_dense_mat                            ... bench:     581,181.50 ns/iter (+/- 23,808.29)
test algorithms::kruskal::test::kruskal_graph_1_2_hash_graph                           ... bench:     147,218.03 ns/iter (+/- 27,481.81)
```

### prim

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


new2
```
test algorithms::prim::test::prim_graph_100_200_adj_list                 ... bench:  32,108,141 ns/iter (+/- 3,496,710)
test algorithms::prim::test::prim_graph_100_200_csr_mat                  ... bench:  30,334,686 ns/iter (+/- 3,055,574)
test algorithms::prim::test::prim_graph_10_200_adj_list                  ... bench:   6,394,235 ns/iter (+/- 139,199)
test algorithms::prim::test::prim_graph_10_200_csr_mat                   ... bench:   6,054,597 ns/iter (+/- 191,380)
test algorithms::prim::test::prim_graph_10_20_adj_list                   ... bench:   1,721,724 ns/iter (+/- 26,669)
test algorithms::prim::test::prim_graph_10_20_csr_mat                    ... bench:   1,707,785 ns/iter (+/- 93,754)
test algorithms::prim::test::prim_graph_10_20_dense_mat                  ... bench:  49,646,724 ns/iter (+/- 364,379)
test algorithms::prim::test::prim_graph_1_200_adj_list                   ... bench:   1,912,484 ns/iter (+/- 9,389)
test algorithms::prim::test::prim_graph_1_20_adj_list                    ... bench:     442,683 ns/iter (+/- 25,148)
test algorithms::prim::test::prim_graph_1_20_csr_mat                     ... bench:     435,747 ns/iter (+/- 65,600)
test algorithms::prim::test::prim_graph_1_20_dense_mat                   ... bench:   1,722,942 ns/iter (+/- 268,333)
test algorithms::prim::test::prim_graph_1_2_adj_list                     ... bench:     108,872 ns/iter (+/- 302)
test algorithms::prim::test::prim_graph_1_2_csr_mat                      ... bench:     105,554 ns/iter (+/- 2,346)
test algorithms::prim::test::prim_graph_1_2_dense_mat                    ... bench:     636,106 ns/iter (+/- 40,673)
```

## nearest neighbor

```
test algorithms::nearest_neighbor::test::nearest_neighbor_k_10_adj_list  ... bench:         811 ns/iter (+/- 135)
test algorithms::nearest_neighbor::test::nearest_neighbor_k_10e_adj_list ... bench:         738 ns/iter (+/- 25)
test algorithms::nearest_neighbor::test::nearest_neighbor_k_12_adj_list  ... bench:       1,035 ns/iter (+/- 28)
test algorithms::nearest_neighbor::test::nearest_neighbor_k_12e_adj_list ... bench:         979 ns/iter (+/- 16)
```

new2

```
test algorithms::nearest_neighbor::test::nearest_neighbor_k_10_adj_list  ... bench:         228.78 ns/iter (+/- 11.92)
test algorithms::nearest_neighbor::test::nearest_neighbor_k_10e_adj_list ... bench:         218.88 ns/iter (+/- 4.05)
test algorithms::nearest_neighbor::test::nearest_neighbor_k_12_adj_list  ... bench:         266.94 ns/iter (+/- 6.79)
test algorithms::nearest_neighbor::test::nearest_neighbor_k_12e_adj_list ... bench:         268.09 ns/iter (+/- 5.54)
```

## branch

```

```

## conversion

```
test graph::conversion::test::read_digraph_100_200_adj_list ... bench:  37,407,896.50 ns/iter (+/- 1,491,658.10)
test graph::conversion::test::read_digraph_10_200_adj_list  ... bench:  35,493,591.90 ns/iter (+/- 2,519,055.60)
test graph::conversion::test::read_digraph_10_20_adj_list   ... bench:   2,944,768.05 ns/iter (+/- 296,310.88)
test graph::conversion::test::read_digraph_1_200_adj_list   ... bench:  42,673,604.90 ns/iter (+/- 2,605,262.60)
test graph::conversion::test::read_digraph_1_20_adj_list    ... bench:   2,967,491.80 ns/iter (+/- 58,780.39)
test graph::conversion::test::read_digraph_1_2_adj_list     ... bench:     260,548.85 ns/iter (+/- 20,632.22)
test graph::conversion::test::read_graph_100_200_adj_list   ... bench:  57,106,970.90 ns/iter (+/- 5,384,408.85)
test graph::conversion::test::read_graph_10_200_adj_list    ... bench:  66,692,513.40 ns/iter (+/- 4,301,225.77)
test graph::conversion::test::read_graph_10_20_adj_list     ... bench:   3,649,486.40 ns/iter (+/- 161,212.30)
test graph::conversion::test::read_graph_1_200_adj_list     ... bench: 104,340,824.40 ns/iter (+/- 13,012,557.80)
test graph::conversion::test::read_graph_1_20_adj_list      ... bench:   4,442,879.80 ns/iter (+/- 437,433.58)
test graph::conversion::test::read_graph_1_2_adj_list       ... bench:     333,235.45 ns/iter (+/- 17,446.75)
```

new

```
test graph::conversion::test::read_digraph_100_200_adj_list ... bench:  28,522,759.40 ns/iter (+/- 1,637,934.78)
test graph::conversion::test::read_digraph_10_200_adj_list  ... bench:  26,717,736.90 ns/iter (+/- 1,462,959.88)
test graph::conversion::test::read_digraph_10_20_adj_list   ... bench:   2,029,833.40 ns/iter (+/- 35,002.39)
test graph::conversion::test::read_digraph_1_200_adj_list   ... bench:  34,756,255.10 ns/iter (+/- 699,784.98)
test graph::conversion::test::read_digraph_1_20_adj_list    ... bench:   2,253,500.60 ns/iter (+/- 72,359.75)
test graph::conversion::test::read_digraph_1_2_adj_list     ... bench:     186,398.42 ns/iter (+/- 13,201.99)
test graph::conversion::test::read_graph_100_200_adj_list   ... bench:  48,316,485.00 ns/iter (+/- 4,841,516.26)
test graph::conversion::test::read_graph_10_200_adj_list    ... bench:  59,306,389.30 ns/iter (+/- 5,799,690.82)
test graph::conversion::test::read_graph_10_20_adj_list     ... bench:   2,890,843.80 ns/iter (+/- 171,367.69)
test graph::conversion::test::read_graph_1_200_adj_list     ... bench: 100,625,181.90 ns/iter (+/- 12,718,056.73)
test graph::conversion::test::read_graph_1_20_adj_list      ... bench:   3,699,509.85 ns/iter (+/- 205,857.13)
test graph::conversion::test::read_graph_1_2_adj_list       ... bench:     266,778.03 ns/iter (+/- 9,874.01)
```

## conversion - kostenminimal

```
test graph::conversion::test::read_kostenminimal1_adj_list       ... bench:       1,030.14 ns/iter (+/- 75.35)
test graph::conversion::test::read_kostenminimal2_adj_list       ... bench:         824.48 ns/iter (+/- 14.47)
test graph::conversion::test::read_kostenminimal3_adj_list       ... bench:       1,001.58 ns/iter (+/- 120.20)
test graph::conversion::test::read_kostenminimal4_adj_list       ... bench:       1,051.00 ns/iter (+/- 101.79)
test graph::conversion::test::read_kostenminimal_gross1_adj_list ... bench:     146,402.80 ns/iter (+/- 465.58)
test graph::conversion::test::read_kostenminimal_gross2_adj_list ... bench:     137,940.51 ns/iter (+/- 3,812.12)
test graph::conversion::test::read_kostenminimal_gross3_adj_list ... bench:     146,598.66 ns/iter (+/- 7,247.43)
```

new

```
test graph::conversion::test::read_kostenminimal1_adj_list       ... bench:         941.95 ns/iter (+/- 171.42)
test graph::conversion::test::read_kostenminimal2_adj_list       ... bench:         781.57 ns/iter (+/- 42.12)
test graph::conversion::test::read_kostenminimal3_adj_list       ... bench:         915.07 ns/iter (+/- 76.64)
test graph::conversion::test::read_kostenminimal4_adj_list       ... bench:         922.05 ns/iter (+/- 147.40)
test graph::conversion::test::read_kostenminimal_gross1_adj_list ... bench:     114,596.27 ns/iter (+/- 1,179.52)
test graph::conversion::test::read_kostenminimal_gross2_adj_list ... bench:     110,006.09 ns/iter (+/- 708.54)
test graph::conversion::test::read_kostenminimal_gross3_adj_list ... bench:     120,262.50 ns/iter (+/- 2,095.97)
```

## cycle canceling

with cloning
```
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_1_adj_list       ... bench:       2,354.07 ns/iter (+/- 52.73)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_1_csr_mat        ... bench:       2,398.11 ns/iter (+/- 29.08)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_2_adj_list       ... bench:       1,892.81 ns/iter (+/- 57.56)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_2_csr_mat        ... bench:       1,828.13 ns/iter (+/- 140.01)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_3_adj_list       ... bench:       1,549.56 ns/iter (+/- 46.75)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_3_csr_mat        ... bench:       1,521.74 ns/iter (+/- 84.66)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_4_adj_list       ... bench:       1,531.30 ns/iter (+/- 20.50)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_4_csr_mat        ... bench:       1,488.24 ns/iter (+/- 15.38)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_1_adj_list ... bench:  87,105,197.20 ns/iter (+/- 4,559,551.99)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_1_csr_mat  ... bench:  57,721,979.00 ns/iter (+/- 241,936.05)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_2_adj_list ... bench:  85,749,215.60 ns/iter (+/- 3,545,354.46)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_2_csr_mat  ... bench:  57,513,056.30 ns/iter (+/- 648,395.39)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_3_adj_list ... bench:   1,047,820.60 ns/iter (+/- 31,695.75)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_3_csr_mat  ... bench:   1,692,701.60 ns/iter (+/- 36,106.01)
```


with edge/node removal
```
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_1_adj_list       ... bench:     322,237.06 ns/iter (+/- 295,705.44)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_1_csr_mat        ... bench:   1,132,159.79 ns/iter (+/- 1,477,045.85)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_2_adj_list       ... bench:     103,736.48 ns/iter (+/- 128,801.64)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_2_csr_mat        ... bench:   1,049,921.16 ns/iter (+/- 1,334,906.63)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_3_adj_list       ... bench:      62,211.26 ns/iter (+/- 76,319.87)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_3_csr_mat        ... bench:   1,016,040.32 ns/iter (+/- 1,278,921.84)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_4_adj_list       ... bench:      62,624.09 ns/iter (+/- 75,910.27)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_4_csr_mat        ... bench:     919,933.53 ns/iter (+/- 1,144,910.53)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_1_adj_list ... bench:  92,822,782.10 ns/iter (+/- 1,202,440.55)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_1_csr_mat  ... bench:  58,409,330.90 ns/iter (+/- 2,094,027.86)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_2_adj_list ... bench:  92,575,892.90 ns/iter (+/- 942,580.34)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_2_csr_mat  ... bench:  57,939,606.50 ns/iter
```

new2
```
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_1_adj_list       ... bench:     224,821.35 ns/iter (+/- 292,190.19)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_1_csr_mat        ... bench:     371,503.32 ns/iter (+/- 463,308.02)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_2_adj_list       ... bench:     256,107.19 ns/iter (+/- 322,415.04)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_2_csr_mat        ... bench:     369,553.42 ns/iter (+/- 466,323.74)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_3_adj_list       ... bench:     263,063.40 ns/iter (+/- 328,194.88)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_3_csr_mat        ... bench:     370,750.43 ns/iter (+/- 456,099.30)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_4_adj_list       ... bench:     209,457.27 ns/iter (+/- 263,603.31)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_4_csr_mat        ... bench:     347,611.80 ns/iter (+/- 439,373.26)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_1_adj_list ... bench:  52,658,066.90 ns/iter (+/- 7,771,818.09)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_1_csr_mat  ... bench:  58,957,740.00 ns/iter (+/- 2,324,129.07)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_2_adj_list ... bench:  53,498,469.00 ns/iter (+/- 7,899,191.58)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_2_csr_mat  ... bench:  57,840,720.60 ns/iter (+/- 1,373,452.70)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_3_adj_list ... bench:     934,554.00 ns/iter (+/- 12,113.11)
test algorithms::cycle_canceling::test::cycle_canceling_kostenminimal_gross_3_csr_mat  ... bench:   1,886,071.00 ns/iter (+/- 192,009.21)
```

## ssp

with cloning
```
test algorithms::ssp::test::ssp_kostenminimal_1_adj_list                               ... bench:       1,638.47 ns/iter (+/- 33.68)
test algorithms::ssp::test::ssp_kostenminimal_1_csr_mat                                ... bench:       1,517.93 ns/iter (+/- 55.33)
test algorithms::ssp::test::ssp_kostenminimal_2_adj_list                               ... bench:         984.95 ns/iter (+/- 22.08)
test algorithms::ssp::test::ssp_kostenminimal_2_csr_mat                                ... bench:         790.70 ns/iter (+/- 20.64)
test algorithms::ssp::test::ssp_kostenminimal_3_adj_list                               ... bench:       1,699.08 ns/iter (+/- 56.85)
test algorithms::ssp::test::ssp_kostenminimal_3_csr_mat                                ... bench:       1,522.43 ns/iter (+/- 44.60)
test algorithms::ssp::test::ssp_kostenminimal_4_adj_list                               ... bench:       1,828.30 ns/iter (+/- 59.28)
test algorithms::ssp::test::ssp_kostenminimal_4_csr_mat                                ... bench:       1,611.78 ns/iter (+/- 46.54)
test algorithms::ssp::test::ssp_kostenminimal_gross_1_adj_list                         ... bench:  24,890,719.00 ns/iter (+/- 335,391.95)
test algorithms::ssp::test::ssp_kostenminimal_gross_1_csr_mat                          ... bench:  19,516,387.80 ns/iter (+/- 1,828,830.83)
test algorithms::ssp::test::ssp_kostenminimal_gross_2_adj_list                         ... bench:  25,719,664.70 ns/iter (+/- 182,623.42)
test algorithms::ssp::test::ssp_kostenminimal_gross_2_csr_mat                          ... bench:  20,009,433.00 ns/iter (+/- 2,611,144.90)
test algorithms::ssp::test::ssp_kostenminimal_gross_3_adj_list                         ... bench:  58,108,510.30 ns/iter (+/- 532,777.48)
test algorithms::ssp::test::ssp_kostenminimal_gross_3_csr_mat                          ... bench:  45,659,820.90 ns/iter (+/- 3,793,378.10)
```

with edge removal
```
test algorithms::ssp::test::ssp_kostenminimal_1_adj_list                               ... bench:       1,722.61 ns/iter (+/- 579.70)
test algorithms::ssp::test::ssp_kostenminimal_1_csr_mat                                ... bench:       1,731.20 ns/iter (+/- 177.46)
test algorithms::ssp::test::ssp_kostenminimal_2_adj_list                               ... bench:         756.75 ns/iter (+/- 58.46)
test algorithms::ssp::test::ssp_kostenminimal_2_csr_mat                                ... bench:         829.62 ns/iter (+/- 19.83)
test algorithms::ssp::test::ssp_kostenminimal_3_adj_list                               ... bench:       1,656.48 ns/iter (+/- 74.39)
test algorithms::ssp::test::ssp_kostenminimal_3_csr_mat                                ... bench:       1,748.56 ns/iter (+/- 199.55)
test algorithms::ssp::test::ssp_kostenminimal_4_adj_list                               ... bench:       1,746.21 ns/iter (+/- 85.01)
test algorithms::ssp::test::ssp_kostenminimal_4_csr_mat                                ... bench:       1,819.42 ns/iter (+/- 69.77)
test algorithms::ssp::test::ssp_kostenminimal_gross_1_adj_list                         ... bench:  17,916,739.50 ns/iter (+/- 335,550.20)
test algorithms::ssp::test::ssp_kostenminimal_gross_1_csr_mat                          ... bench:  23,220,633.70 ns/iter (+/- 669,892.92)
test algorithms::ssp::test::ssp_kostenminimal_gross_2_adj_list                         ... bench:  18,433,344.50 ns/iter (+/- 1,409,764.53)
test algorithms::ssp::test::ssp_kostenminimal_gross_2_csr_mat                          ... bench:  24,046,658.40 ns/iter (+/- 757,248.16)
test algorithms::ssp::test::ssp_kostenminimal_gross_3_adj_list                         ... bench:  42,696,112.30 ns/iter (+/- 802,340.55)
test algorithms::ssp::test::ssp_kostenminimal_gross_3_csr_mat                          ... bench:  53,162,328.80 ns/iter (+/- 568,786.09)
```
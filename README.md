# Graph library

TODO

implement adapt function on concrete graphs do not create trait for it as not possible.
The adapt function should take self<N, W> and return Self<N2, W2>. The type parameters
are infered by the Adaptor trait which specifies the Output Nodes and Wheihts.
Additionally an view trait which does not change the fundamental graph but only offers
the possibility to filter, complement etc. the graph.

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
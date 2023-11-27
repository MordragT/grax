# Graph library

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

with updating flow_map

```
test edmonds_karp::test::edmonds_karp_fluss2_adj_list        ... bench:      98,009 ns/iter (+/- 180)
test edmonds_karp::test::edmonds_karp_fluss2_adj_mat         ... bench:      70,413 ns/iter (+/- 974)
test edmonds_karp::test::edmonds_karp_fluss_adj_list         ... bench:      19,774 ns/iter (+/- 747)
test edmonds_karp::test::edmonds_karp_fluss_adj_mat          ... bench:      15,018 ns/iter (+/- 205)
test edmonds_karp::test::edmonds_karp_g_1_2_adj_list         ... bench: 710,934,308 ns/iter (+/- 22,641,626)
test edmonds_karp::test::edmonds_karp_g_1_2_adj_mat          ... bench: 501,081,800 ns/iter (+/- 31,451,721)

```

without

```
test edmonds_karp::test::edmonds_karp_fluss2_adj_list        ... bench:      10,048 ns/iter (+/- 25)
test edmonds_karp::test::edmonds_karp_fluss2_adj_mat         ... bench:      12,355 ns/iter (+/- 98)
test edmonds_karp::test::edmonds_karp_fluss_adj_list         ... bench:       4,318 ns/iter (+/- 20)
test edmonds_karp::test::edmonds_karp_fluss_adj_mat          ... bench:       4,760 ns/iter (+/- 2,159)
test edmonds_karp::test::edmonds_karp_g_1_2_adj_list         ... bench:   1,909,591 ns/iter (+/- 26,767)
test edmonds_karp::test::edmonds_karp_g_1_2_adj_mat          ... bench:  31,828,876 ns/iter (+/- 3,334,849)

```
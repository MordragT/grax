# Graph library

## Benchmarks

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
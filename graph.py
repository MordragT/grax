import queue
from math import inf

class UnionFind:
    def __init__(self, nodes):
        self.parent = nodes
        self.size = [1 for _ in (0..nodes.size)]

    def find(self, needle):
        root = needle
        path = []

        while self.parent[root] != root:
            path.push(root)
            root = self.parent[root]

        for node in path:
            self.parent[node] = root

        return root

    def union(self, x, y):
        root_x = self.find(x)
        root_y = self.find(y)

        if root_x == root_y:
            return

        if self.size[root_x] < self.size[root_y]:
            tmp = root_x
            root_x = root_y
            root_y = tmp

        self.parent[root_y] = root_x
        self.size[root_x] += self.size[root_y]

class Graph:

    def __init__(self, edges, number_of_nodes):
        self.edges = edges
        self.number_of_nodes = number_of_nodes

    def prim(self, start_node):
        visited = [False for i in range(self.number_of_nodes)]
        pq = queue.PriorityQueue()
        weight_map = [inf for i in range(self.number_of_nodes)]
        weight_map[start_node] = 0

        total_weight = 0

        pq.put((0.0, start_node))

        while not pq.empty():
            node_weight, node = pq.get()
            if visited[node] is True:
                continue

            visited[node] = True
            total_weight += node_weight

            for edge in self.edges[node]:
                if visited[edge.position] is False:
                    min_weight = weight_map[edge.position]
                    if min_weight > edge.weight:
                        weight_map[edge.position] = edge.weight
                        pq.put((edge.weight, edge.position))

        return total_weight

    def kruksal(self):
        pq = queue.PriorityQueue()

        for i, edge in enumerate(self.edges):
            # i ist from index
            pq.put((edge.weight, (i, edge.position)))

        union_find = UnionFind()
        total_weight = 0

        while not pq.empty():
            weight, parent, to = pq.get()
            if union_find.find(parent) == union_find.find(to):
                continue

            union_find.union(parent, to)
            total_weight += weight

        return total_weight


class Edge:

    def __init__(self, weight, position):
        self.weight = weight
        self.position = position


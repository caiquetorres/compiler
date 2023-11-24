fun main() {
    let visited: [bool; 100];
    let graph: [[i32; 100]; 100];

    let nodesAmount = 4;
    let edges = [
        [0, 1, 1, 0],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [0, 1, 1, 0]
    ];

    for i in 0..nodesAmount {
        for j in 0..nodesAmount {
            graph[i][j] = edges[i][j];
        }
    }

    dfs(0, nodesAmount, visited, graph);
}

fun dfs(node: i32, nodesAmount: i32, visited: [bool; 100], graph: [[i32; 100]; 100]) {
    visited[node] = true;
    println node;

    for i in 0..nodesAmount {
        if graph[node][i] == 1 && !visited[i] {
            dfs(i, nodesAmount, visited, graph);
        }
    }
}

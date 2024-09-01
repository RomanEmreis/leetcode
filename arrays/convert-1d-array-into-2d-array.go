func construct2DArray(original []int, m int, n int) [][]int {
    if len(original) != m * n {
        return make([][]int, 0);
    }

    result := make([][]int, m);
    for i := 0; i < m; i++ {
        result[i] = make([]int, n);
    }

    for i := 0; i < len(original); i++ {
        result[i / n][i % n] = original[i];
    }

    return result;
}

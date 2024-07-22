/*
  You are given an array of strings names, and an array heights that consists of distinct positive integers. Both arrays are of length n.
  For each index i, names[i] and heights[i] denote the name and height of the ith person.
  Return names sorted in descending order by the people's heights.

  Example 1:
    Input: names = ["Mary","John","Emma"], heights = [180,165,170]
    Output: ["Mary","Emma","John"]
    Explanation: Mary is the tallest, followed by Emma and John.
  
  Example 2:
    Input: names = ["Alice","Bob","Bob"], heights = [155,185,150]
    Output: ["Bob","Alice","Bob"]
    Explanation: The first Bob is the tallest, followed by Alice and the second Bob.
*/
type Person struct {
    Name string;
    Height int;
}

func sortPeople(names []string, heights []int) []string {
    n := len(names);
    people := make([]Person, n);

    for i := 0; i < n; i++ {
        people[i] = Person{Name: names[i], Height: heights[i]};
    }

    sort.Slice(people, func(i, j int) bool {
        return people[i].Height > people[j].Height;
    });

    result := make([]string, n);
    for i := 0; i < n; i++ {
        result[i] = people[i].Name;
    }

    return result;
}

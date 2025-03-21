/*
  2115. Find All Possible Recipes from Given Supplies
  
  You have information about n different recipes. You are given a string array recipes and a 2D string array ingredients. The ith recipe has the name recipes[i], and you can create it if you have all the needed ingredients from ingredients[i]. A recipe can also be an ingredient for other recipes, i.e., ingredients[i] may contain a string that is in recipes.
  You are also given a string array supplies containing all the ingredients that you initially have, and you have an infinite supply of all of them.
  
  Return a list of all the recipes that you can create. You may return the answer in any order.
  
  Note that two recipes may contain each other in their ingredients.
  
  Example 1:
  Input: recipes = ["bread"], ingredients = [["yeast","flour"]], supplies = ["yeast","flour","corn"]
  Output: ["bread"]
  Explanation:
  We can create "bread" since we have the ingredients "yeast" and "flour".
  
  Example 2:
  Input: recipes = ["bread","sandwich"], ingredients = [["yeast","flour"],["bread","meat"]], supplies = ["yeast","flour","meat"]
  Output: ["bread","sandwich"]
  Explanation:
  We can create "bread" since we have the ingredients "yeast" and "flour".
  We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
  
  Example 3:
  Input: recipes = ["bread","sandwich","burger"], ingredients = [["yeast","flour"],["bread","meat"],["sandwich","meat","bread"]], supplies = ["yeast","flour","meat"]
  Output: ["bread","sandwich","burger"]
  Explanation:
  We can create "bread" since we have the ingredients "yeast" and "flour".
  We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
  We can create "burger" since we have the ingredient "meat" and can create the ingredients "bread" and "sandwich".
*/
public class Solution {
    public IList<string> FindAllRecipes(string[] recipes, IList<IList<string>> ingredients, string[] supplies) {
        Dictionary<string, List<string>> graph = [];
        Dictionary<string, int> inDegree = [];

        for (int i = 0; i < recipes.Length; ++i) {
            foreach (var ing in ingredients[i]) {
                if (graph.ContainsKey(ing)) graph[ing].Add(recipes[i]);
                else graph[ing] = [recipes[i]];
            }
            inDegree[recipes[i]] = ingredients[i].Count;
        }

        Queue<string> q = [];
        foreach (var s in supplies) {
            q.Enqueue(s);
        }

        List<string> result = [];
        while (q.Count > 0) {
            for (int i = q.Count; i > 0; --i) {
                string sup = q.Dequeue();
                foreach (var rec in graph.GetValueOrDefault(sup, [])) {
                    if (--inDegree[rec] == 0) {
                        result.Add(rec);
                        q.Enqueue(rec);
                    }
                }
            }
        }

        return result;
    }
}

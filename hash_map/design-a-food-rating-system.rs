/*
  2353. Design a Food Rating System
  
  Design a food rating system that can do the following:
  
  Modify the rating of a food item listed in the system.
  Return the highest-rated food item for a type of cuisine in the system.
  Implement the FoodRatings class:
  FoodRatings(String[] foods, String[] cuisines, int[] ratings) Initializes the system. The food items are described by foods, cuisines and ratings, all of which have a length of n.
  foods[i] is the name of the ith food,
  cuisines[i] is the type of cuisine of the ith food, and
  ratings[i] is the initial rating of the ith food.
  void changeRating(String food, int newRating) Changes the rating of the food item with the name food.
  String highestRated(String cuisine) Returns the name of the food item that has the highest rating for the given type of cuisine. If there is a tie, return the item with the lexicographically smaller name.
  Note that a string x is lexicographically smaller than string y if x comes before y in dictionary order, that is, either x is a prefix of y, or if i is the first position such that x[i] != y[i], then x[i] comes before y[i] in alphabetic order.
  
  Example 1:
  Input
  ["FoodRatings", "highestRated", "highestRated", "changeRating", "highestRated", "changeRating", "highestRated"]
  [[["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"], ["korean", "japanese", "japanese", "greek", "japanese", "korean"], [9, 12, 8, 15, 14, 7]], ["korean"], ["japanese"], ["sushi", 16], ["japanese"], ["ramen", 16], ["japanese"]]
  Output
  [null, "kimchi", "ramen", null, "sushi", null, "ramen"]
  Explanation
  FoodRatings foodRatings = new FoodRatings(["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"], ["korean", "japanese", "japanese", "greek", "japanese", "korean"], [9, 12, 8, 15, 14, 7]);
  foodRatings.highestRated("korean"); // return "kimchi"
                                      // "kimchi" is the highest rated korean food with a rating of 9.
  foodRatings.highestRated("japanese"); // return "ramen"
                                        // "ramen" is the highest rated japanese food with a rating of 14.
  foodRatings.changeRating("sushi", 16); // "sushi" now has a rating of 16.
  foodRatings.highestRated("japanese"); // return "sushi"
                                        // "sushi" is the highest rated japanese food with a rating of 16.
  foodRatings.changeRating("ramen", 16); // "ramen" now has a rating of 16.
  foodRatings.highestRated("japanese"); // return "ramen"
                                        // Both "sushi" and "ramen" have a rating of 16.
                                        // However, "ramen" is lexicographically smaller than "sushi". 
*/
use std::collections::{HashMap, BTreeSet};

struct FoodRatings {
    food_to_cuisine: HashMap<String, String>,
    food_to_rating: HashMap<String, i32>,
    ratings: HashMap<String, BTreeSet<(i32, String)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {

    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_to_cuisine = HashMap::new();
        let mut food_to_rating = HashMap::new();
        let mut cuisine_ratings: HashMap<String, BTreeSet<(i32, String)>> = HashMap::new();

        for i in 0..foods.len() {
            let food = foods[i].clone();
            let cuisine = cuisines[i].clone();
            let rating = ratings[i];

            food_to_cuisine.insert(food.clone(), cuisine.clone());
            food_to_rating.insert(food.clone(), rating);

            cuisine_ratings
                .entry(cuisine)
                .or_insert_with(BTreeSet::new)
                .insert((-rating, food));
        }

        Self {
            food_to_cuisine,
            food_to_rating,
            ratings: cuisine_ratings,
        }
    }
    
    fn change_rating(&mut self, food: String, new_rating: i32) {
        if let Some(cuisine) = self.food_to_cuisine.get(&food) {
            if let Some(&old_rating) = self.food_to_rating.get(&food) {
                if let Some(rset) = self.ratings.get_mut(cuisine) {
                    rset.remove(&(-old_rating, food.to_string()));
                    rset.insert((-new_rating, food.to_string()));
                }
                self.food_to_rating.insert(food.to_string(), new_rating);
            }
        }
    }
    
    fn highest_rated(&self, cuisine: String) -> String {
        self.ratings
            .get(cuisine.as_str())
            .and_then(|rset| rset.iter().next().map(|(_, food)| food.clone()))
            .unwrap()
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */

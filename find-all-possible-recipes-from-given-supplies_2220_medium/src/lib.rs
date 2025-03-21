use std::collections::HashSet;

// https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/
#[allow(dead_code)]
struct Solution;

// #[allow(dead_code)]
// impl Solution {
//     pub fn find_all_recipes(
//         recipes: Vec<String>,
//         ingredients: Vec<Vec<String>>,
//         supplies: Vec<String>,
//     ) -> Vec<String> {
//         todo!()
//     }
// }

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let recipes_set: HashSet<_> = recipes.iter().cloned().collect();
        let supplies_set: HashSet<_> = supplies.into_iter().collect();
        let mut result = Vec::with_capacity(recipes.len());

        for recipe in recipes.iter() {
            let mut visited = HashSet::new();
            if dfs(
                &ingredients,
                &recipes,
                &recipes_set,
                &supplies_set,
                recipe,
                &mut visited,
            ) {
                result.push(recipe.clone());
            }
        }
        result
    }
}

fn dfs(
    ingredients: &[Vec<String>],
    recipes: &[String],
    recipes_set: &HashSet<String>,
    supplies_set: &HashSet<String>,
    cur_ingredient: &String,
    visited: &mut HashSet<String>,
) -> bool {
    if supplies_set.contains(cur_ingredient) {
        return true;
    }

    let Some(pos) = recipes.iter().position(|r| r == cur_ingredient) else {
        return false;
    };

    if !visited.insert(cur_ingredient.clone()) {
        return false;
    }

    let can_make = ingredients[pos].iter().all(|ingredient| {
        dfs(
            ingredients,
            recipes,
            recipes_set,
            supplies_set,
            ingredient,
            visited,
        )
    });

    visited.remove(cur_ingredient);

    can_make
}

// TLE
// #[allow(dead_code)]
// impl Solution {
//     pub fn find_all_recipes(
//         recipes: Vec<String>,
//         ingredients: Vec<Vec<String>>,
//         supplies: Vec<String>,
//     ) -> Vec<String> {
//         let mut result = Vec::with_capacity(recipes.len());
//         for (idx, recipe) in recipes.iter().enumerate() {
//             let cur_ingredients = ingredients[idx].clone();
//             if dfs(
//                 cur_ingredients,
//                 &ingredients,
//                 &recipes,
//                 &supplies,
//                 &mut HashSet::new(),
//             ) {
//                 // dbg!(idx, &recipe, &cur_ingredients);
//                 result.push(recipe.clone());
//             }
//         }
//         result
//     }
// }

// fn dfs(
//     cur_ingredients: Vec<String>,
//     ingredients: &[Vec<String>],
//     recipes: &[String],
//     supplies: &[String],
//     visited: &mut HashSet<String>,
// ) -> bool {
//     // if all cur_ingredients in supplies return true
//     // dbg!(&cur_ingredients);
//     if cur_ingredients
//         .iter()
//         .all(|cur_ingredient| supplies.contains(cur_ingredient))
//     {
//         // dbg!("true");
//         // dbg!(&cur_ingredients);
//         return true;
//     }

//     for cur_ingredient in cur_ingredients.iter() {
//         if !visited.insert(cur_ingredient.clone()) {
//             return false;
//         }
//     }

//     let mut filtered_ingredients = Vec::new();

//     for cur_ingredient in cur_ingredients.iter() {
//         if supplies.contains(cur_ingredient) {
//             continue;
//         }

//         if let Some(pos) = recipes.iter().position(|recipe| recipe == cur_ingredient) {
//             filtered_ingredients.extend(ingredients[pos].iter().cloned());
//         } else {
//             return false;
//         }
//     }

//     for cur_ingredient in cur_ingredients.iter() {
//         visited.remove(cur_ingredient);
//     }

//     if dfs(
//         filtered_ingredients,
//         ingredients,
//         recipes,
//         supplies,
//         visited,
//     ) {
//         true
//     } else {
//         false
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let recipes = vec!["bread".to_string()];
        let ingredients = vec![vec!["yeast".to_string(), "flour".to_string()]];
        let supplies = vec!["yeast".to_string(), "flour".to_string(), "corn".to_string()];
        let output = vec!["bread".to_string()];
        let result = Solution::find_all_recipes(recipes, ingredients, supplies);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let recipes = vec!["bread".to_string(), "sandwich".to_string()];
        let ingredients = vec![
            vec!["yeast".to_string(), "flour".to_string()],
            vec!["bread".to_string(), "meat".to_string()],
        ];
        let supplies = vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()];
        let output = vec!["bread".to_string(), "sandwich".to_string()];
        let result = Solution::find_all_recipes(recipes, ingredients, supplies);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let recipes = vec![
            "bread".to_string(),
            "sandwich".to_string(),
            "burger".to_string(),
        ];
        let ingredients = vec![
            vec!["yeast".to_string(), "flour".to_string()],
            vec!["bread".to_string(), "meat".to_string()],
            vec![
                "sandwich".to_string(),
                "meat".to_string(),
                "bread".to_string(),
            ],
        ];
        let supplies = vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()];
        let output = vec![
            "bread".to_string(),
            "sandwich".to_string(),
            "burger".to_string(),
        ];
        let result = Solution::find_all_recipes(recipes, ingredients, supplies);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let recipes = vec![
            "ju".to_string(),
            "fzjnm".to_string(),
            "x".to_string(),
            "e".to_string(),
            "zpmcz".to_string(),
            "h".to_string(),
            "q".to_string(),
        ];
        let ingredients = vec![
            vec!["d".to_string()],
            vec!["hveml".to_string(), "f".to_string(), "cpivl".to_string()],
            vec![
                "cpivl".to_string(),
                "zpmcz".to_string(),
                "h".to_string(),
                "e".to_string(),
                "fzjnm".to_string(),
                "ju".to_string(),
            ],
            vec![
                "cpivl".to_string(),
                "hveml".to_string(),
                "zpmcz".to_string(),
                "ju".to_string(),
                "h".to_string(),
            ],
            vec![
                "h".to_string(),
                "fzjnm".to_string(),
                "e".to_string(),
                "q".to_string(),
                "x".to_string(),
            ],
            vec![
                "d".to_string(),
                "hveml".to_string(),
                "cpivl".to_string(),
                "q".to_string(),
                "zpmcz".to_string(),
                "ju".to_string(),
                "e".to_string(),
                "x".to_string(),
            ],
            vec!["f".to_string(), "hveml".to_string(), "cpivl".to_string()],
        ];
        let supplies = vec![
            "f".to_string(),
            "hveml".to_string(),
            "cpivl".to_string(),
            "d".to_string(),
        ];
        let output = vec!["ju".to_string(), "fzjnm".to_string(), "q".to_string()];
        let result = Solution::find_all_recipes(recipes, ingredients, supplies);
        assert_eq!(result, output);
    }
}

use std::collections::HashSet;
use std::io;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

// file to hashmap converter
fn file_to_hashmap(filename:&str) -> io::Result<HashMap<String,Vec<String>>> {
    let file: File = File::open(filename)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines(){
        let line: String = line?;

        if let Some((key,values)) = line.split_once(":") {
            let value_list: Vec<String> = values.split(",").map(|s: &str|s.trim().to_lowercase()).filter(|s: &String| !s.is_empty()).into_iter().collect();
            map.insert(key.trim().to_lowercase(), value_list);
        }
    }
    return Ok(map)
}

// returns weather a given name likes a given recipe
fn name_likes_recipe(name: &String, recipe: &String, names_and_categories: &HashMap<String, Vec<String>>, recipes_and_ingredients: &HashMap<String, Vec<String>>, ingredients_and_categories: &HashMap<&String, &String>) -> Result<bool, String> {

    //checks if either name or recipe don't exits and returns an Err
    if !names_and_categories.contains_key(name){
        return Err("Name does not exist".to_string());
    } else if !recipes_and_ingredients.contains_key(recipe){
        return Err("Recipe does not exist".to_string());
    }

    let liked_categories: Vec<&String> = names_and_categories.get(name).unwrap().into_iter().collect();

    let ingredients_in_recipe: Vec<&String> = recipes_and_ingredients.get(recipe).unwrap().into_iter().collect();

    let mut categories_in_recipe: HashSet<&String> = HashSet::new();

    // get the categories that the recipe has
    for ingredient in ingredients_in_recipe{
        // check if the ingredient is in a category of the liked categories
        if  ingredients_and_categories.contains_key(ingredient){
            categories_in_recipe.insert(ingredients_and_categories.get(ingredient).unwrap());
        }    
    }

    let mut num_liked_categories: i32 = 0; 

    let total_categories_in_recipe: usize = categories_in_recipe.len();

    let ingredients_in_recipe: Vec<&String> = recipes_and_ingredients.get(recipe).unwrap().into_iter().collect();

    for ingredient in ingredients_in_recipe{
        // check if the ingredient is in a category of the liked categories
        if  ingredients_and_categories.contains_key(ingredient){
            categories_in_recipe.insert(ingredients_and_categories.get(ingredient).unwrap());
        }       
    }

    for category in categories_in_recipe{
        if liked_categories.contains(&category){
            num_liked_categories += 1;
        }
    }

    // operation to divide and round up
    let good: f32 = ((num_liked_categories as f32) / (total_categories_in_recipe as f32) * 100.0).ceil();

    let mut likes_recipe: bool = false;

    if good >= 60.0{
        likes_recipe = true;
    }

    return Ok(likes_recipe);
}

//returns a list of recipes liked by name
fn liked_recipes(name: &String, names_and_categories: &HashMap<String, Vec<String>>, recipes_and_ingredients: &HashMap<String, Vec<String>>, ingredients_and_recipes: &HashMap<&String, &String>) -> Result<Vec<String>,String> {

    if !names_and_categories.contains_key(name){
        return Err("Name does not exist".to_string())
    }

    let mut liked_list: Vec<String> = Vec::new();

    for recipe in recipes_and_ingredients.keys() {
        if name_likes_recipe(name, recipe, names_and_categories,  recipes_and_ingredients, ingredients_and_recipes).unwrap(){
            liked_list.push(recipe.to_string());
        }
    }
    return Ok(liked_list);
}

//returns the top 3 most liked recipes in order
fn popular_recipes(names_and_categories: &HashMap<String, Vec<String>>, recipes_and_ingredients: &HashMap<String, Vec<String>>, ingredients_and_categories: &HashMap<&String, &String>) -> (String, i32, String, i32, String, i32) {

    let names: Vec<&String> = names_and_categories.keys().collect();

    let recipes: Vec<&String> = recipes_and_ingredients.keys().collect();

    // make variables for the top 3 and their like counts
    let mut  top_1: String = String::new();
    let mut top_1_likes: i32 = 0;

    let mut top_2: String = String::new();
    let mut  top_2_likes: i32 = 0;

    let mut top_3: String = String::new();
    let mut top_3_likes: i32 = 0;

    for recipe in &recipes{
        let mut likes: i32 = 0;
        for name in &names{

            if name_likes_recipe(name, recipe, &names_and_categories, &recipes_and_ingredients, &ingredients_and_categories).unwrap() == true {
                likes += 1;
            }
        }
        if likes > top_3_likes{

            top_3 = recipe.to_string();
            top_3_likes = likes;

            if top_3_likes > top_2_likes {

                std::mem::swap(&mut top_3, &mut top_2);
                std::mem::swap(&mut top_3_likes, &mut top_2_likes);

                if top_2_likes > top_1_likes {
    
                    std::mem::swap(&mut top_2, &mut top_1);
                    std::mem::swap(&mut top_2_likes, &mut top_1_likes);
                }
            }
        }
    }
    
    // puts the top 3 in order if any of the top 3 have the same number of likes
    if top_3_likes == top_2_likes {
        if top_3 > top_2{
            std::mem::swap(&mut top_3, &mut top_2);
            std::mem::swap(&mut top_3_likes, &mut top_2_likes);
        }
    }
    if top_2_likes == top_1_likes {
        if top_2 > top_1{
            std::mem::swap(&mut top_2, &mut top_1);
            std::mem::swap(&mut top_2_likes, &mut top_1_likes);
        }
    }
    if top_3_likes == top_2_likes {
        if top_3 > top_2{
            std::mem::swap(&mut top_3, &mut top_2);
            std::mem::swap(&mut top_3_likes, &mut top_2_likes);
        }
    }
    return ( top_1, top_1_likes, top_2, top_2_likes, top_3, top_3_likes);
    
}



fn main() {
    // create hashmaps from the given files
    let cai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/categories_ingredients.txt").expect("Failed to read the file.");
    let nac: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/people_categories.txt").expect("Failed to read the file.");
    let rai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/recipes.txt").expect("Failed to read the file.");

    //creates reverse hashmap with keys being ingredients and value being the category
    let mut iac: HashMap<&String, &String> = HashMap::new();
    for category in cai.keys(){
        for ingredient in cai.get(category).unwrap(){
            iac.insert(ingredient,category);
        }
    }

    // get user input for which function to run
    let mut input1: String = String::new();
    println!("Enter Command (name), (popular recipes), (name,recipe): ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let input: String = input1.trim().to_lowercase();

    // decide which function to run based on input
    if input.contains(","){ //input with comma means name and recipe signifying name_likes_recipe function
        let name_and_recipe: Vec<&str> = (input.split(", ")).collect();
        let name: String = name_and_recipe[0].to_string();
        let recipe: String = name_and_recipe[1].to_string();
        println!("{} likes {}: {:?}",&name, &recipe, name_likes_recipe(&name, &recipe, &nac, &rai, &iac));
    } else if input == "popular recipes".to_string(){   //input of "Popular recipes" means popular_recipes function
        println!("The most liked recipes are: {:?}", popular_recipes(&nac, &rai, &iac));
    } else { // anything else means just name signifying Liked_recipes function
        println!("{} likes these recipes: {:?}", &input, liked_recipes(&input, &nac, &rai, &iac));
    }
    
}


#[test]
fn name_likes_recipe_works_true(){
    let test_rai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_recipes_and_ingredients.txt").expect("Failed to read the file.");
    let test_nac: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_names_and_categories.txt").expect("Failed to read the file.");
    let cai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/categories_ingredients.txt").expect("Failed to read the file.");

    let mut iac: HashMap<&String, &String> = HashMap::new();
    for category in cai.keys(){
        for ingredient in cai.get(category).unwrap(){
            iac.insert(ingredient,category);
        }
    }
    
    let result = name_likes_recipe(&("riwaz shrestha".to_string()), &("test_recipe".to_string()), &test_nac, &test_rai, &iac);
    let expected = Ok(true);
    assert_eq!(result, expected, "Fail!!!");
}

#[test]
fn name_likes_recipe_works_false(){
    let test_rai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_recipes_and_ingredients.txt").expect("Failed to read the file.");
    let test_nac: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_names_and_categories.txt").expect("Failed to read the file.");
    let cai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/categories_ingredients.txt").expect("Failed to read the file.");

    let mut iac: HashMap<&String, &String> = HashMap::new();
    for category in cai.keys(){
        for ingredient in cai.get(category).unwrap(){
            iac.insert(ingredient,category);
        }
    }
    
    let result = name_likes_recipe(&("liam dunn".to_string()), &("test_recipe".to_string()), &test_nac, &test_rai, &iac);
    let expected = Ok(false);
    assert_eq!(result, expected, "Fail!!!");
}

#[test]
fn name_likes_recipe_fails_at_name(){
    let test_rai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_recipes_and_ingredients.txt").expect("Failed to read the file.");
    let test_nac: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_names_and_categories.txt").expect("Failed to read the file.");
    let cai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/categories_ingredients.txt").expect("Failed to read the file.");

    let mut iac: HashMap<&String, &String> = HashMap::new();
    for category in cai.keys(){
        for ingredient in cai.get(category).unwrap(){
            iac.insert(ingredient,category);
        }
    }
    
    let result = name_likes_recipe(&("first last".to_string()), &("test_recipe".to_string()), &test_nac, &test_rai, &iac);
    let expected = Err("Name does not exist".to_string());
    assert_eq!(result, expected, "Fail!!!");
}

#[test]
fn name_likes_recipe_fails_at_recipe(){
    let test_rai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_recipes_and_ingredients.txt").expect("Failed to read the file.");
    let test_nac: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_names_and_categories.txt").expect("Failed to read the file.");
    let cai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/categories_ingredients.txt").expect("Failed to read the file.");

    let mut iac: HashMap<&String, &String> = HashMap::new();
    for category in cai.keys(){
        for ingredient in cai.get(category).unwrap(){
            iac.insert(ingredient,category);
        }
    }
    
    let result = name_likes_recipe(&("liam dunn".to_string()), &("test_recipe_fail".to_string()), &test_nac, &test_rai, &iac);
    let expected = Err("Recipe does not exist".to_string());
    assert_eq!(result, expected, "Fail!!!");
}

#[test]
fn liked_recipes_works(){
    let test_rai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_recipes_and_ingredients.txt").expect("Failed to read the file.");
    let test_nac: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_names_and_categories.txt").expect("Failed to read the file.");
    let cai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/categories_ingredients.txt").expect("Failed to read the file.");

    let mut iac: HashMap<&String, &String> = HashMap::new();
    for category in cai.keys(){
        for ingredient in cai.get(category).unwrap(){
            iac.insert(ingredient,category);
        }
    }
    
    let result = liked_recipes(&("liam dunn".to_string()), &test_nac, &test_rai, &iac);
    let expected: Result<Vec<String>, String> = Ok(vec!["test_recipe2".to_string()]);
    assert_eq!(result, expected, "Fail!!!");
}

#[test]
fn popular_recipes_works(){
    let test_rai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_recipes_and_ingredients.txt").expect("Failed to read the file.");
    let test_nac: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/test_names_and_categories.txt").expect("Failed to read the file.");
    let cai: HashMap<String, Vec<String>> = file_to_hashmap("/Users/RiwazShrestha/Desktop/DS210/Homework/HW6/hw_06_new/categories_ingredients.txt").expect("Failed to read the file.");

    let mut iac: HashMap<&String, &String> = HashMap::new();
    for category in cai.keys(){
        for ingredient in cai.get(category).unwrap(){
            iac.insert(ingredient,category);
        }
    }
    
    let result: (String, i32, String, i32, String, i32) = popular_recipes(&test_nac, &test_rai, &iac);
    let expected: (String, i32, String, i32, String, i32) = ("test_recipe2".to_string(), 2, "test_recipe".to_string(), 1, "".to_string(), 0);
    assert_eq!(result, expected, "Fail!!!");
}
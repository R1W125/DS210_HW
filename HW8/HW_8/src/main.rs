use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use csv;

#[derive(Debug, Clone)]
enum ColumnVal {
    One(String),
    Two(bool),
    Three(f64),
    Four(i64),
}

#[derive(Debug, Clone)]
struct DataFrame {
    titles: Vec<String>,
    df: HashMap< String, Vec<ColumnVal>>, 
    num_rows: i32,
    title_types: Vec<u32>,
}

// For returning errors
#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}

impl DataFrame {
    fn new() -> Self {
        DataFrame{
            titles: Vec::new(),
            df: HashMap::new(),
            num_rows: 0,
            title_types: Vec::new(),
        }
    }

    fn read_csv(&mut self, path: &str, types: &Vec<u32>) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_path(path)?;
        let mut first_row = true;
        for result in rdr.records() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let r = result.unwrap();
            let mut row: Vec<ColumnVal> = vec![];
            if first_row {
                self.titles = r.iter().map(|s| s.to_string()).collect();
                first_row = false;
                continue;
            }
            for (i, elem) in r.iter().enumerate() {
                match types[i] {
                    1 => row.push(ColumnVal::One(elem.to_string())),
                    2 => row.push(ColumnVal::Two(elem.parse::<bool>().unwrap())),
                    3 => row.push(ColumnVal::Three(elem.parse::<f64>().unwrap())),
                    4 => row.push(ColumnVal::Four(elem.parse::<i64>().unwrap())),
                    _ => return Err(Box::new(MyError("Unknown type".to_string()))),
                }
            }
            // Put the data into the dataframe
            for (title, value) in self.titles.iter().zip(row.iter()) {
                self.df.entry(title.clone()).or_insert_with(Vec::new).push(value.clone());
            }
            self.num_rows += 1;
            self.title_types = types.clone();
        }
        Ok(())
    }

    fn print(&self) {
        for title in &self.titles{
            print!("{:<15}", title);
        }
        println!();
        for i in 0..self.num_rows as usize {
            for title in &self.titles {
                if let Some(column) = self.df.get(title) {
                    match &column[i] {
                        ColumnVal::One(s) => print!("{:<15}", s),
                        ColumnVal::Two(b) => print!("{:<15}", b),
                        ColumnVal::Three(f) => print!("{:<15}", f),
                        ColumnVal::Four(i) => print!("{:<15}", i),
                    }
                } else {
                    print!("None ");
                }
            }
            println!();
        }    
    }

    fn add_column(&mut self, name:String, col: Vec<ColumnVal>) -> Result<DataFrame, MyError> {
        if col.len() == self.num_rows as usize {
            self.titles.push(name.clone());
            self.df.insert(name, col);
            Ok(self.clone())
        } else {
            Err(MyError(format!("Columns aren't of the same length!!")))
        }
    }

    // Need to add more function parameters and fix the return type
    fn merge_frame(&mut self, df2: DataFrame) -> Result<DataFrame, MyError> {
        if self.titles == df2.titles && self.title_types == df2.title_types {
            for title in self.titles.iter() {
                self.df.get_mut(title).unwrap().extend(df2.df.get(title).unwrap().iter().cloned());
            }
            self.num_rows += df2.num_rows;
            Ok(self.clone())
        } else {
            Err(MyError(format!("Columns or column types don't match!!")))
        }
    }

    // Need to add more function parameters and fix the return type
    fn restrict_columns(&mut self, cols: Vec<String>) -> Result<DataFrame, MyError> {
        for i in &cols {
            if self.titles.contains(&i) {
                continue;
            } else {
                return Err(MyError(format!("Column does not exist '{}'!!", i)));
            }
        }
        let mut restricted_df: DataFrame = DataFrame::new();
        let mut types: Vec<u32> = Vec::new();
        for i in &cols{
            for j in 0..self.titles.len(){
                if *i == self.titles[j]{
                    types.push(self.title_types[j])
                }
            }
        }
        restricted_df.num_rows = self.num_rows;
        restricted_df.title_types = types;
        restricted_df.titles.extend(cols.clone());
        for k in cols{
            restricted_df.df.insert(k.clone().to_string(), self.df.get(&k).unwrap().clone());
        }
        Ok(restricted_df)
    }

    fn filter(&mut self, label: &str, operation: fn(&ColumnVal) -> bool) -> Result<DataFrame, MyError> {
        if !self.titles.contains(&label.to_string()){
            return Err(MyError(format!("Column does not exist '{}'!!", label)));
        }
        let mut clone: DataFrame = self.clone();
        let column = self.df.get(label).unwrap();
        for i in (0..(self.num_rows)).rev(){  
            let val = &column[i as usize];
            if operation(val) {
                continue;
            } else {
                clone.num_rows -= 1;
                for j in &clone.titles{
                    clone.df.get_mut(j).unwrap().remove(i as usize);
                }               
            }
        }
        Ok(clone)
    }

    fn column_op(&mut self, labels: &[String], op: fn(&[Vec<ColumnVal>]) -> Vec<ColumnVal>) -> Vec<ColumnVal> {
        let mut cols: Vec<Vec<ColumnVal>> = Vec::new();
        for label in labels {
            cols.push(self.df.get(label).unwrap().clone()); 
        }
        op(&cols)
    }

    fn median(&mut self, label: &str) -> f64 {
        let labels = vec![label.to_string()];
    
        let op = |cols: &[Vec<ColumnVal>]| -> Vec<ColumnVal> {
            let col = &cols[0];
            let mut floats: Vec<f64> = col.iter().filter_map(|val| {
                if let ColumnVal::Three(f) = val {
                    Some(*f)
                } else {
                    None
                }
            }).collect();
    
            floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
            let len = floats.len();
            if floats.len() == 0 {
                panic!("Cannot find median of empty column");
            }
    
            let median = if len % 2 == 1 {
                floats[len / 2]
            } else {
                (floats[len / 2 - 1] + floats[len / 2]) / 2.0
            };
    
            vec![ColumnVal::Three(median)]
        };
    
        let result = self.column_op(&labels, op);
    
        if let ColumnVal::Three(f) = result[0] {
            f
        } else {
            panic!("Median operation did not return a float");
        }
    }

    fn sub_columns(&mut self, labels: &[String]) -> Vec<ColumnVal> {
        let op = |cols: &[Vec<ColumnVal>]| -> Vec<ColumnVal> {
            if cols.len() != 2 {
                panic!("Expected 2 columns");
            }

            let col1 = &cols[0];
            let col2 = &cols[1];

            let mut result = Vec::new();

            for (v1, v2) in col1.iter().zip(col2.iter()) {
                let num1 = match v1 {
                    ColumnVal::Four(i) => *i,
                    _ => panic!("Not a number"),
                };
                let num2 = match v2 {
                    ColumnVal::Four(i) => *i,
                    _ => panic!("Not a number"),
                };
                result.push(ColumnVal::Four(num1 - num2));
                
            }
            result
        };
        let output = self.column_op(labels, op);
        output
    }
}

fn main() {
    let mut df1: DataFrame = DataFrame::new();
    let types = vec![1,4,3,4,4,2];
    df1.read_csv("DF1.csv", &types).expect("Failed to read CSV");
    println!("Print Data Frame:");
    println!();
    df1.print();

    let mut df2: DataFrame = DataFrame::new();
    let types = vec![1,4,3,4,4,2];
    df2.read_csv("DF2.csv", &types).expect("Failed to read CSV");
    println!();
    println!("Print Data Frame 2:");
    println!();
    df2.print();
    println!();

    println!("Print Merged Data Frame:");
    println!();
    let merged = df1.merge_frame(df2);
    merged.unwrap().print();

    let hof: Vec<ColumnVal> = vec![
        ColumnVal::Two(false),
        ColumnVal::Two(false),
        ColumnVal::Two(true),
        ColumnVal::Two(false),
        ColumnVal::Two(false),
        ColumnVal::Two(false),
        ColumnVal::Two(false),
    ];
    let new_column: String = "Hall of Fame".to_string();

    println!();
    println!("Add Hall of Fame Column:");
    println!();
    df1.add_column(new_column, hof).unwrap();
    df1.print();
    println!();

    let restricted = df1.restrict_columns(vec!["Name".to_string(), "TotalPoints".to_string()]).unwrap();
    println!("Restricted Data Frame (by name and total points):");
    println!();
    restricted.print();

    let over_25_ppg = |val: &ColumnVal| match val {
        ColumnVal::Three(f) => *f > 25.0,
        _ => false, 
    };
    println!();
    println!("Filetered Data Frame (over 25 ppg):");
    let filtered_df = df1.filter("PPG", over_25_ppg);
    println!();
    (filtered_df.unwrap()).print();
    
    println!();
    println!("Median of PPG: {}",df1.median("PPG")); 
    println!();

    let labels = vec!["TotalPoints".to_string(), "YearBorn".to_string()];
    let subbed = df1.sub_columns(&labels);
    println!("TotalPoints - BirthYear:");
    println!();
    for i in subbed{
        match i {
            ColumnVal::Four(x) => println!("{}", x),
            _ => panic!("SUM WENT RONG"),
        }
    }
}
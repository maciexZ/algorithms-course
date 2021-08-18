use std::vec;


//Insert sort
// sorting inplace
pub fn insert_sort(vector: &mut Vec<i32>){
    let n = vector.len();
    
    for i in 1..n{
        let number = vector[i];
        let mut j = i;
        while j>0 && number < vector[j-1]{
            vector[j] = vector[j-1];
            vector[j-1] = number; 
            j-=1;
        }
    }
//end insert_sort
}
//------------------------------------------------------------------

//bubble sort
//sorting inplace
pub fn bubble_sort(vector:&mut Vec<i32>){
    let mut alert = 1;
    while alert !=0 {
        alert = 0;
        for i in 1..vector.len(){
            if vector[i]<vector[i-1]{
                let number = vector[i];
                vector[i] = vector[i-1];
                vector[i-1] = number;
                alert +=1;
            }
        }
    }
//end bubble_sort    
}
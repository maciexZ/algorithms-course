

/*
karatsuba algorithm 

two numbers must be the same lenght. If not, padding is necessary.  
In case of odd lenght m = floor(n/2) but the index of cuting should be ceil(n/2)
*/
pub fn karatsuba(x:i64, y:i64)-> i64 {
    if x < 10 || y < 10 {
        (x * y) as i64
    } else {
        //int to string
        let mut x:Vec<char> = format!("{}", x).chars().collect();
        let mut y:Vec<char> = format!("{}", y).chars().collect();
        //lenght of two numbers must be equal
        while x.len() != y.len(){
            if x.len() > y.len(){
                y= add_padding(y);
            } else{
                x = add_padding(x);
            }
        }
        //lenght 
        let m = x.len() as u32 /2;
        let cuting_index = x.len() /2 + (x.len()%2);

        //string cut and string to int    
        let a:i64= *(&x[..cuting_index].iter().collect::<String>().parse().unwrap());
        let b:i64 = *(&x[cuting_index..].iter().collect::<String>().parse().unwrap());
        let c:i64 = *(&y[..cuting_index].iter().collect::<String>().parse().unwrap());
        let d:i64= *(&y[cuting_index..].iter().collect::<String>().parse().unwrap());

        //recursive call
        let ac = karatsuba(a, c);
        let bd = karatsuba(b, d);
        let ad_plus_bc  = karatsuba(a+b, c+d) - ac - bd;

        //karatsuba magic
        // println!("a{} b{} c{} d{}  ac {}, n {}, ad_plus_bc {}, bd {}",a, b, c, d, ac, n, ad_plus_bc, bd);
        ac * i64::pow(10,2*m) + (ad_plus_bc * i64::pow(10, m)) + bd 
    }

//end fn    
}    

//adding char '0' to vector
fn add_padding(mut v:Vec<char>) -> Vec<char> {
    v.splice(0..0, ['0']);
    v
}

//end karatsuba -----------------------------------------------------------------------------

//MERGE SORT --------------------------------------------------------------------------------
pub fn merge_sort(vector: &Vec<i32>)->Vec<i32>{
    //additonal layer allows return owned object
    let mut input_vector = vector.clone();
    
    //index of last value from array
    let r = input_vector.len()-1;

    //reall merge sort - non public function
    merge_sort_inner(&mut input_vector, 0, r);

    input_vector
//end merge_sort
}

//merging overrides input vector with sorted numbers
fn merge(input_vector:&mut Vec<i32>, l:usize, m:usize, r:usize) {
       
        
    //variables to be used by loops
    let mut k = l;
    let mut i = 0;
    let mut j = 0;

    //lenghts of parts
    let n1 = m - l+1;
    let n2 = r - m;

    //parting the array, creates temporary vectors
    let (left, right) = (input_vector[l..l+n1].to_vec(), input_vector[(l+n1)..r+1].to_vec()); 

    //choosing smaller numbers
    while i<n1 && j<n2 {
        if left[i] <= right[j]{
            input_vector[k] = left[i];
            i+=1;

        }else{
            input_vector[k]= right[j];
            j+=1;
        }
        k+=1;
    }

    //adding reminders
    while i<n1 {
        input_vector[k] = left[i];
        i+=1;
        k+=1;
    }
    while j<n2 {
        input_vector[k] = right[j];
        j+=1;
        k+=1;
    }
//end merge
}

/*
inner merge_sort (this function really sorting a vector)
input - mutable reference to vector, 
returns void sorts values in original vector
*/
fn merge_sort_inner(input_vector:&mut Vec<i32>, l:usize, r:usize){
    //base case
    if l < r {
    
    //halving. m - index of first value in second half (index from input_vector)
    let m = l + (r - l) / 2;
    //divide
    merge_sort_inner(input_vector, l, m);
    merge_sort_inner(input_vector, m+1 , r);
    //conquer
    merge(input_vector, l,  m, r);

    }
//end merge_sort_inner  
}
//END MERGE_SORT----------------------------------------------------------------







//--------------------------------------------------------------------------------------------
//tests
#[cfg(test)]
mod tests {

use super::*;

//karatsuba tests
#[test]
fn eq_lenght(){
assert_eq!(karatsuba(1122, 3344),3751968 )
}

#[test]
fn non_eq_lenght(){
assert_eq!(karatsuba(1122, 113344),127171968)
}

#[test]
fn zero_multiplication(){
assert_eq!(karatsuba(0, 113344),0)
}

#[test]
fn negative_number(){
assert_eq!(karatsuba(-1111, 113344),-125925184)
}
//merge_sort tests

#[test]
fn even_number_of_items(){
    assert_eq!(merge_sort(&vec![6,5,4,3,2,1]), vec![1,2,3,4,5,6])
}

#[test]
fn odd_number_of_items(){
    assert_eq!(merge_sort(&vec![1,7,6,5,3,2,4]), vec![1,2,3,4,5,6,7])
}

}
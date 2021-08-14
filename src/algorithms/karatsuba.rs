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



//tests
#[cfg(test)]
mod tests {

use super::*;

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

}
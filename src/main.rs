use std::io;
fn main() {
    //Loan Amt
    println!("Enter the Loan Amount");
    let mut loan_amt = String::new();
    io::stdin()
    .read_line(&mut loan_amt).unwrap();
    let loan_amt: f64=loan_amt.trim().parse().unwrap(); 
    
    //Interest Rate
    println!("Enter the Rate of interest");
    let mut rate_interest = String::new();
    io::stdin()
    .read_line(&mut rate_interest).unwrap();
    let rate_interest: f64=rate_interest.trim().parse().unwrap();

    let r: f64 =(rate_interest)/1200.0;

    //Time Period
    println!("Enter the Time period");
    let mut n=String::new();
    io::stdin()
    .read_line(&mut n).unwrap();
    let n: f64 =n.trim().parse().unwrap(); 


    println!("Loan Amount {loan_amt}");
    println!("Time period {n}");
    println!("Monthly Rate of Interest {r}");

    //Emi Calculation
    let emi=(loan_amt*r*(1.0+r).powf(n))/((1.0+r).powf(n)-1.0);
    println!("{}",emi);

}

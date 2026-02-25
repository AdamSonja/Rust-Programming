fn main() {
    let loan_amt: f64=13810.0;
    let interest_rate: f64=8.12;
    let r=interest_rate/12.0/100.0;
    let time_period: f64=10.0*12.0;

    let emi: f64=(loan_amt*r*(1.0+ r)
    .powf(time_period))/((1.0+r)
    .powf(time_period)-1.0);
    println!("{}",emi);
}

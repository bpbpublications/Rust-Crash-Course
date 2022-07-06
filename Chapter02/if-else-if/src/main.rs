fn main() {
    let marks = 56;
    print_grade(marks);
}

fn print_grade(marks: u8){
    if marks >= 90{
        println!("Grade is A");
    } else if marks >= 80{
        println!("Grade is A-");
    } else if marks >= 70{
        println!("Grade is B");
    } else if marks >= 60{
        println!("Grade is B-");
    } else if marks >= 50{
        println!("Grade is C");
    } else if marks >= 40{
        println!("Grade is C-");
    } else if marks >= 30{
        println!("Grade is D");
    } else {
        println!("Grade is F");
    }
}

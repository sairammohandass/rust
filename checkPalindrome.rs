//palindrome

fn main(){
    
    let s= String::from("🦀madam🦀");
    println!("{}",ispalindrome(&s));
    
}

fn ispalindrome(s:&str)->bool{

let ch:Vec<char>=s.trim().chars().filter(|c| !c.is_whitespace()).collect();
for i in 0..ch.len()/2 {
    
    if ch[i]!=ch[ch.len()-1-i]
    {
        return false;
    }
}
    return true;
}

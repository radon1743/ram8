use std::fs;

//as the name suggest, it decodes the opcode 
fn decode(ram:&mut Vec<i8>,
    part:&str,
    acc:&mut i8,
    prog_count:&mut i8) {    
    let opcode=&part[0..3];
    let addr:i8=part[4..].trim().parse().expect("not a integer");

    if opcode=="LOD" {
        *acc=ram[addr as usize];        
    }
    
    else if opcode=="ADD" {
        *acc+=ram[addr as usize];

        
    }
    
    else if opcode=="STR" {
        ram[addr as usize]=*acc;
        
    }
    
    else if opcode=="JMP" {
        *prog_count=addr as i8;
    }
    
    else if opcode=="HLT"{
        *prog_count=-2;
    }
    
    else if opcode=="INC" {
        ram[addr as usize]+=1;
    }
    
    else if opcode=="OTP" {
        println!("{}",ram[addr as usize]);
    }
    
    else if opcode=="ITP" {
        *acc=addr;
    }
    *prog_count+=1;
    fetch(ram,
         acc,
         prog_count);
}

//this function fetches input from 
//a text file named input.txt 
//and feeds into another function(decode)
fn get_current_working_dir() -> String {
    let res = std::env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}
fn fetch(ram:&mut Vec<i8>, 
    acc:&mut i8,
    prog_count:&mut i8){
    //file path 
    let mut file_path=get_current_working_dir();
    file_path.push_str("/input.txt");  
    //println!("{}",file_path);
    //this is whole txt as Sting
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //to iter over each line
    let mut inst:Vec<&str>=Vec::new();
    for part in contents.lines(){
        inst.push(part);
    }
    if *prog_count>=0{
        decode(ram,
            inst[*prog_count as usize],
            //reg_a,reg_b,reg_c, reg_d,
            acc,
            prog_count)
    }
    else {
        println!("Program Halted");
    }

}

//program starts from here 
fn main() {
    //println!("Hello, world!");

    //this hashmap represents ram
    let mut ram: Vec<i8>;
    ram=vec![0;16];
    //program counter for easy Jump operation 
    let mut prog_count:i8=0;

    //accumulator kind of temparory variable
    let mut acc:i8=0;

    fetch(&mut ram,
        &mut acc,
        &mut prog_count);
}

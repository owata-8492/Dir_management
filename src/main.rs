use std::io;
use std::fs::File;
use std::process::exit;

const MAX_LENGTH: u16 = 1024;
const MAX_ITEM: u16 = 10000;

struct Date{
       year : u16,
       month: u8,
       day  : u8,
}

struct Item{
       id : u32,
       name : String,
       date : Date,
       address : String,
       others : String,
}

fn store_new_item(x: &String, list: &mut Vec<Item>){

   if x.match_indices(",").count() != 4 { println!("Error! Please input again.\n"); return;}
   let tmp: Vec<&str> = x.split(',').collect();

   if tmp[2].match_indices("-").count() != 2 { println!("Error! Please input again.\n"); return;}
   let tmp_date: Vec<&str> = tmp[2].split('-').collect();

   match tmp[0].parse::<u32>(){
   	 Ok(n) => (),
   	 Err(err) => {println!("Error! Please input again.\n");return},
   };

   match tmp_date[0].parse::<u16>(){
   	 Ok(n) => (),
   	 Err(err) => {println!("Error! Please input again.\n");return},
   };

   match tmp_date[1].parse::<u8>(){
   	 Ok(n) => (),
   	 Err(err) => {println!("Error! Please input again.\n");return},
   };

   match tmp_date[2].parse::<u8>(){
   	 Ok(n) => (),
   	 Err(err) => {println!("Error! Please input again.\n");return},
   };

   let one_date = Date{
      year: tmp_date[0].parse().unwrap(),
      month: tmp_date[1].parse().unwrap(),
      day: tmp_date[2].parse().unwrap(),
   };

   let one_list = Item{ 
       id: tmp[0].parse().unwrap(),
       name: tmp[1].to_string(),
       date: one_date,
       address: tmp[3].to_string(),
       others: tmp[4].to_string(),
   };
   list.push(one_list);

   println!("Input complete!");
}

fn cmd_q(){
   println!("Command Q start!\n");
  exit(0);
}

fn cmd_c(list: &Vec<Item>){
   println!("Command C start!\n");
   println!("There are {} items\n", list.len())
}

fn cmd_p(list: &mut Vec<Item>){
   println!("Command P start!\n");

   let mut start = 0;
   let mut end =list.len();
   
   for i in (start as usize)..(end as usize){
       let printing_list = &list[i];
       println!("ID:{}",printing_list.id);
       println!("Name:{}",printing_list.name);
       println!("Birth:{}-{}-{}",printing_list.date.year, printing_list.date.month, printing_list.date.day);
       println!("Address:{}",printing_list.address);
       println!("Others:{}\n",printing_list.others);
       }
}

fn cmd_h(){
   println!("Command H start!\n"); 
   println!("CSV data : Input ID, name , date, address, other coments");
   println!("      Ex : 123456, ABC School, 2000-1-23, ABC street 123, hoge");
   println!("Quit this proglam         : Input %Q"); 
   println!("Check the number of items : Input %C");
   println!("Print all of items        : Input %P\n");
}

fn main() {
    println!("Help:Input %H");

    let mut list: Vec<Item> = Vec::new();
    let mut date: Vec<Date> = Vec::new();

    loop{
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("Failed to read line");
	line = line.replace("\n","\0");
	if line.len() <= 1 {println!("Error! Please input again.\n"); continue;}
	
	let cmd = line.chars().take(2).collect::<String>();
	let cmd_s: &str = &cmd;
	
	if line.len() >= (MAX_LENGTH as usize) {println!("Error! Input line is too long.\n"); continue;};
	if list.len() >= (MAX_ITEM as usize) {println!("Error! Profile list is full.\n"); continue;};
	
	match cmd_s {
	      "%Q" => {println!("Triggered %Q!"); cmd_q();},
	      "%C" => {println!("Triggered %C!"); cmd_c(&list);},
	      "%P" => {println!("Triggered %P!"); cmd_p(&mut list);},
	      "%H" => {println!("Triggered %H!"); cmd_h();},
	      _ if cmd.starts_with("%") => println!("Command {} is not exist! Please input again.\n",&line),
	      _ => {println!("Input new item\n");store_new_item(&line,&mut list);},
	      }
	}
}

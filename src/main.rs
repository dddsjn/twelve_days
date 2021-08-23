fn main() {
	
    let days = ["first", "second", "third", "forth", "fifth", "sixth", 
	            "seventh", "eighth", "ninth", "tenth", "eleventh", "twelth"];
		
    let gift_list = ["a partridge in a pear tree", "Two turtle doves", 
	                 "Three french hens", "Four calling birds", "Five golden rings", 
					 "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", 
					 "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping",
					 "Twelve drummers drumming"];

    let mut counter = 0;
	
    for day in days {
		
        println!("On the {} of Christmas my true love gave to me", day);
				
	    if counter == 0 {
			println!("{}", gift_list[0]);
			counter = counter + 2;
		} else {
			
			let gift_slice = &gift_list[1..counter];
			
			for gift in gift_slice.iter().rev() {
				println!("{}", gift);	
			}
			
			println!("and {}", gift_list[0]);
			counter = counter + 1;
		}
	}
	
}

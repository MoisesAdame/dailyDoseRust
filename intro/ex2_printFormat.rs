fn main(){

	// Like python's format technique
	println!("Chinese president's name is {0}{1}{2}.", "习", "金", "平");

	// With variable names
	println!("Hello my name is {name} {middleName} {lastName}.",
			 name = "Moisés",
			 middleName = "Adame",
			 lastName = "Aguilar");

	// Imprimir en diferentes formatos
	println!("Base 10: {}", 420);
	println!("Base 2: {:b}", 420);
	println!("Base 8: {:o}", 420);
	println!("Base 16m: {:x}", 420);
	println!("Base 16M: {:X}", 420);




	

}
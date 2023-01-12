// Author: Moisés Adame-Aguilar
// Date: January 11, 2023
// Description: Traits.

// Definition: A way to tell the compiler that some types share the same behavior.

// Traits can only be implemented if either the type or the tarit is local to the crate.

// Defining a trait.
pub trait Summary{
	// Adding deault behaviour.
	fn summarize(&self){
		println!("Watch the story...");
		println!("");
	}
}

// Defining each stuct.
pub struct New{
	header: String,
	description: String,
	author: String,
	full_story: String,
	views: u32
}

pub struct Tweet{
	author: String,
	description: String,
	full_tweet: String,
}

pub struct TikTok{
	author: String,
	description: String,
}

// Defining each stuct's Summary implementation.
impl Summary for New{
	fn summarize(&self){
		println!("{}: {}. By: {}({})", self.header, self.description, self.author, self.views);
		println!("");
	}
}

impl Summary for Tweet{
	fn summarize(&self){
		println!("{} - {}", self.description, self.author);
		println!("");
	}
}

// To implement a default trait, just leave the braces empty.
impl Summary for TikTok{}

// Defining suplementary methods.
impl New{
	pub fn new(header: String, description: String, author: String, full_story: String, views: u32) -> New{
		New{
			header,
			description,
			author,
			full_story,
			views
		}
	}
}

impl Tweet{
	pub fn new(author: String, description: String, full_tweet: String) -> Tweet{
		Tweet{
			author,
			description,
			full_tweet
		}
	}
}

impl TikTok{
	pub fn new(author: String, description: String) -> TikTok{
		TikTok{
			author,
			description
		}
	}
}

// Trait bounds, allow generics but for certain types (the ones with a given trait)

pub fn breaking_news<T: Summary>(story: &T){
	println!("Breaking news!");
	story.summarize();
}
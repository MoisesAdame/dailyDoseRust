// Author: Moisés Adame-Aguilar
// Date: January 11, 2023
// Description: Using traits as a module.

#![allow(dead_code)]

// Importing the module.
pub mod news;
use news::Summary;

fn main(){
	let new1: news::New = news::New::new(String::from("Ataque contra Ciro Gómez Leyva: 11 presuntos responsables son detenidos durante cateos"),
							 String::from("Las detenciones se hicieron en una serie de cateos llevados a cabo por elementos de seguridad capitalinos."),
							 String::from("El Financiero"),
							 String::from("11 personas fueron detenidas por su presunta relación con el atentado contra el periodista Ciro Gómez Leyva, informó este miércoles Claudia Sheinbaum, jefa de Gobierno.
							 La funcionaria explicó que las detenciones se hicieron en 12 cateos simultáneos hechos por la Secretaría de Seguridad Ciudadana y la Fiscalía General de Justicia de la Ciudad de México, con apoyo del Centro Nacional de Inteligencia.
							 “Reitero, no habrá impunidad. En breve más información”, añadió en un mensaje publicado en su cuenta de Twitter."),
							 28);

	let tweet1: news::Tweet = news::Tweet::new(String::from("Ye"),
								   String::from("5G antennas are causing covid-19."),
								   String::from("5G antennas are causing covid-19."));


	let tiktok1: news::TikTok = news::TikTok::new(String::from("Publimetro"),
								   String::from("POV: El metro se quema."));


	println!("== NEWS ==");
	new1.summarize();

	tweet1.summarize();

	tiktok1.summarize();

	news::breaking_news::<news::New>(&new1);
}
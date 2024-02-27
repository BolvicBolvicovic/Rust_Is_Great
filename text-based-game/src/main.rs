use std::{
	io,
	io::{Read, Write},
	thread,
	time::Duration,
	process::exit,
};
use rand::Rng;

pub enum GameState {
	Start,
}

pub enum Action {
	UseItem,
	VisitNewRoom,
	BackToPreviousRoom,
	SearchRoom,
}

impl Action {
	fn show_items(player: &Player) {
		print_epic(format!("{:#?}", player.items));
	}

	fn use_item(game: &mut Game) {
		fn match_item(item: Item) {
			match item {
					Item::UnknownPostion	|
					Item::HeavyHealthPotion	|
					Item::HeavyPoison		|
					Item::MediumHealthPotion|
					Item::MediumPoison		|
					Item::LightHealthPotion	|
					Item::LightPoison 		=> item.drink(game.player);
					Item::SmallKey			|
					Item::MediumKey			|
					Item::LongKey			=> item.try_use(game.player);
			}		
		}

		self.show_items(&game.player);
		print_epic("\nChoisis l'item a utilise via son index.\nSi tu ne veux pas utiliser un item, appuie sur entre.\n");
		let prompt = game.get_prompt();
		match prompt.as_str().next() {
			Some('0') => {
				let item = game.player.items.remove(0);
				match_item(item);
			}
			Some('1')	=> {
				if game.player.items.len() > 0 {
					let item = game.player.items.remove(1);
					match_item(item);
				} else {
					print_epic("There is no object at index 1.\n");
				}
			}
			Some('2')	=> {
				if game.player.items.len() > 1 {
					let item = game.player.items.remove(2);
					match_item(item);
				} else {
					print_epic("There is no object at index 2.\n");
				} 
			}
			Some('3')	=> {
				if game.player.items.len() > 2 {
					let item = game.player.items.remove(3);
					match_item(item);
				} else {
					print_epic("There is no object at index 3.\n");
				} 
			}
			Some('4')	=> {
				if game.player.items.len() > 3 {
					let item = game.player.items.remove(4);
					match_item(item);
				} else {
					print_epic("There is no object at index 4.\n");
				} 
			}
			Some('5')	=> {
				if game.player.items.len() > 4 {
					let item = game.player.items.remove(5);
					match_item(item);
				} else {
					print_epic("There is no object at index 5.\n");
				} 
			}
			Some('6')	=> {
				if game.player.items.len() > 5 {
					let item = game.player.items.remove(6);
					match_item(item);
				} else {
					print_epic("There is no object at index 6.\n");
				} 
			}
			Some('7')	=> {
				if game.player.items.len() > 6 {
					let item = game.player.items.remove(7);
					match_item(item);
				} else {
					print_epic("There is no object at index 7.\n");
				} 
			}
			Some('8')	=> {
				if game.player.items.len() > 7 {
					let item = game.player.items.remove(8);
					match_item(item);
				} else {
					print_epic("There is no object at index 8.\n");
				} 
			}
			Some('9')	=> {
				if game.player.items.len() > 8 {
					let item = game.player.items.remove(9);
					match_item(item);
				} else {
					print_epic("There is no object at index 9.\n");
				} 
			}
			None | Some(_)	=> { }
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Item {
	LightHealthPotion,
	LightPoison,
	MediumHealthPotion,
	MediumPoison,
	HeavyHealthPotion,
	HeavyPoison,
	UnknownDrink,
	SmallKey,
	MediumKey,
	LongKey,
	Dagger,
	Dart,
}

impl Item {
	fn drink_potion(&mut self, hp: i16, player: &mut Player, potion_type: &str) {
		match player.health {
			100 => { println!(
				"You are drinking a {} potion... But it does not do anything.", potion_type); }
			_ => {
				println!(
				"You are drinking a {} potion... And it makes you feel a tiny bit better.", potion_type); }
		}
		player.health += hp;
		if player.health > 100 { player.health = 100; }
	}
	
	fn drink_poison(&mut self, hp: i16, player: &mut Player, poison_type: &str) {
		println!("You are drinking a {} poison... And it makes you feel worse.", poison_type);
		player.health -= hp;
		if player.health > 0 { player.is_dead = true; }
	}

	fn drink(&mut self, player: &mut Player) {
		if *self == Item::UnknownDrink {
			let mut rng = rand::thread_rng();
			let rand: u8 = rng.gen_range(0..=5);
			match rand % 5 {
				0				=> { *self = Item::HeavyPoison;			}
				1				=> { *self = Item::LightPoison;			}
				2				=> { *self = Item::MediumHealthPotion;	}
				3				=> { *self = Item::MediumPoison;		}
				4				=> { *self = Item::HeavyHealthPotion;	}
				5_u8..=u8::MAX	=> { *self = Item::LightHealthPotion;	}
			}
		}
		match *self {
			Item::LightHealthPotion => { self.drink_potion(10, player, "tiny red"); }
			Item::LightPoison => { self.drink_poison(10, player, "small smelling purple"); }
			Item::MediumHealthPotion => { self.drink_potion(20, player, "medium tempting green"); }
			Item::MediumPoison => { self.drink_poison(20, player, "medium darkened aweful"); }
			Item::HeavyHealthPotion => { self.drink_potion(50, player, "full golden godlike"); }
			Item::HeavyPoison => { self.drink_poison(50, player, "most disgusting blueish"); }
			_ => { println!("You cannot drink this item !"); }
		}
	}

	/*fn try_use(target: &mut Item) {
		match self {
			
		}
	}*/
}

pub struct Player {
	pub coins: i16,
	pub health: i16,
	pub is_dead: bool,
	pub items: Vec<Item>,
//TODO Make a list for the item you know (which means if you don't know it you have to try it first)
}

impl Player {
	pub fn new() -> Player {
		Player {
			coins: 0,
			health: 5,
			is_dead: false,
			items: vec![],
		}
	}

	fn add_item(&mut self, item: Item) {
		if self.items.len() < 11 {
			self.items.push(item);
		}
	}

	fn get_item_index(&mut self, item: &Item) -> i16 {
		let mut index = 0;
		for i in self.items.iter() {
			match i {
				item => { break; }
				_ => { index += 1; }
			}
		}
		index
	} 
}

pub fn wait(time: i16) {
	thread::sleep(Duration::from_millis(time.try_into().unwrap()));
}

pub fn print_epic(my_str: &str) {
	for i in my_str.chars() {
		print!("{}",i);
		io::stdout().flush().unwrap();
		wait(10);
	}
	println!("");
}


pub struct Game {
	state: GameState,
	iterations: i16,
	player: Player,
}

impl Game {
	pub fn new() -> Game {
		Game {
			state: GameState::Start,
			iterations: 0,
			player: Player::new(),
		}
	}

	fn get_prompt() -> String {
		print!("🦀 "); io::stdout().flush().unwrap();
		let mut prompt = String::new();
		io::stdin().read_line(&mut prompt).unwrap();
		prompt
	}

	fn intro(&mut self) -> Item {
		print_epic("Tes yeux s'ouvrent doucement mais les tenebres restent.
C'est l'odeur qui guide tes premiers pas.
Une odeur inconnue, la nausee, la faim, la nausee...
Tu mets les mains sur un petit coffre qui etait surplombe d'une petite bougie de cire.
Il est poussiereux. Il sent fort. Tu as faim.");
		wait(2000);
		print_epic("\n\t(entre le numero propose pour faire un choix)\n
1: forcer le coffre\n2: chercher la clef\n");
		loop {
			let prompt = Game::get_prompt();
			match prompt.as_str().chars().next() {
				Some('1') => {
					print_epic("\nTu forces le coffre et il s'ouvre comme un coquillage.
A l'interieur se trouve une etrange fiole contenant un liquide. Tu le mets dans ta poche.");
					return Item::UnknownDrink;
				}
				Some('2') => {
					print_epic("\nTu te baisses et cherches autour, il n'y a rien...
Tu te sens faible.");
					self.player.health -= 1;
					if self.player.health <= 0 {
						print_epic("TU ES MORT.");
						exit(1);
					}
				}
				None | Some(_) => { continue; }
			}
		}
	}

	fn loop_(&mut self) {
		let intro = self.intro();
		self.player.add_item(intro);

		fn display_possibilities() -> Option<Action> {
			print_epic("1: Visiter une nouvelle salle.");
			print_epic("2: Retourner a la salle precedente.");
			print_epic("3: Fouiller la salle.");
			if self.items.len() > 0 {
				print_epic("4: Utilise un item.");
			}
			let prompt = self.get_prompt();
			match prompt.as_str().chars().next() {
				Some('1')		=> Some(Action::VisitNewRoom),
				Some('2')		=> Some(Action::BackToPreviousRoom),
				Some('3')		=> Some(Action::SearchRoom),
				Some('4')		=> Some(Action::UseItem),
				None | Some(_)	=> None,
			}
		}

		loop {
			self.iteration += 1;
			let action = display_possibilities();
			match action {
				Some(Action::UseItem)				=> { action.use_item(self.player);				}
				Some(Action::VisitNewRoom)			=> { action.visit_new_room(self.player);		}
				Some(Action::BackToPreviousRoom)	=> { action.back_to_previous_room(self.player);	}
				Some(Action::SearchRoom)			=> { action.search_room(self.player);			}
				None								=> { continue;									}
			}
			if self.iteration == 10 { println!("You survived the whole game"); break; }
		}
	}
}

fn main() {
	let mut game = Game::new();
	game.loop_();
}

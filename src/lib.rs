use std::str::FromStr;

use std::time::Instant;

use rand::thread_rng;
use rand::seq::SliceRandom;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn start_game<T, U>(controller: T, state_presenter: U) 
where 
    T: Prompter,
    U: Visualizer,
{
    let figure_solution = FigureInput::new_random();
    let start_time = Instant::now();
    
    let mut has_won = false;

    println!("solution: {:?}", figure_solution);

    for _ in 0..6 {
        let answer = controller.get_input("Your answer: ");
        let comparison = answer.cmp(&figure_solution);

        state_presenter.present_game_state(&comparison);
        
        if comparison.is_all_black() {
            has_won = true;
            break;
        }
    }
    
    if has_won {
        println!("You won!");
    }

    let elapsed_time = start_time.elapsed();

    println!("Time needed: {}", elapsed_time.as_secs());
    println!("Waiting for user input: ");
}

pub trait Prompter {
    fn get_input(&self, message_to_show: &str) -> FigureInput;
}

pub trait Visualizer {
    fn present_game_state(&self, hitOutput: &HitOutput);
}


#[derive(Clone, Copy, PartialEq, EnumIter, Debug)]
pub enum FigureColour {
    Red,
    Orange,
    Yellow,
    Blue,
    Purple,
    Green,
}

impl FromStr for FigureColour {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Red),
            "O" => Ok(Self::Orange),
            "Y" => Ok(Self::Yellow),
            "B" => Ok(Self::Blue),
            "P" => Ok(Self::Purple),
            "G" => Ok(Self::Green),
            _ => Err("Incorrect character colour FigureColour."),
        }
    }
}

impl ToString for FigureColour {
    fn to_string(&self) -> String {
        match self {
            FigureColour::Red => "Red".to_string(),
            FigureColour::Orange => "Orange".to_string(),
            FigureColour::Yellow => "Yellow".to_string(),
            FigureColour::Blue => "Blue".to_string(),
            FigureColour::Purple => "Purple".to_string(),
            FigureColour::Green => "Green".to_string(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum HitColour {
    Black,
    White,
    Empty,
}

impl ToString for HitColour {
    fn to_string(&self) -> String {
        match self {
            Self::Black => "Black".to_string(),
            Self::White => "White".to_string(),
            Self::Empty => "Empty".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct HitOutput {
    hits: [HitColour; 4],
}

impl HitOutput {
    fn is_all_black(&self) -> bool {
        self.hits.iter().all(|&colour| { colour == HitColour::Black })
    }
}

#[derive(Debug)]
pub struct FigureInput {
    figures: [FigureColour; 4],
}

impl FigureInput {
    pub fn new(colors: &[FigureColour]) -> FigureInput {
        debug_assert!(colors.len() == 4, "Colors not of len 4.");

        let mut buffer = [FigureColour::Blue; 4];
        for i in 0..buffer.len() {
            buffer[i] = colors[i];
        }

        FigureInput { figures: buffer }
    }

    fn new_random() -> FigureInput {
        let mut rng = rand::thread_rng();
        
        let values: Vec<FigureColour> = FigureColour::iter().collect();
        
        let mut buffer = [FigureColour::Blue; 4];
        let selected_values = values.choose_multiple(&mut rng, buffer.len()).zip(buffer.iter_mut());

        for (selected, buffer_slot) in  selected_values {
            *buffer_slot = *selected;
        }
        
        FigureInput { figures: buffer }
    }

    fn cmp(&self, other: &FigureInput) -> HitOutput {
        let mut output = HitOutput { hits: [HitColour::Empty; 4] };

        for i in 0..self.figures.len() {
            if self.figures[i] == other.figures[i] {
                output.hits[i] = HitColour::Black;
            }
        }

        for i in 0..output.hits.len() {
            if output.hits[i] == HitColour::Empty && other.figures.contains(&self.figures[i]){
                output.hits[i] = HitColour::White;
            }
        }

        // shuffle result so that it's not predictable after multiple game runs
        let mut rng = thread_rng();
        output.hits.shuffle(&mut rng);

        output
    }
}



// -------------

pub struct TerminalPrompter {

}

impl Prompter for TerminalPrompter {
    fn get_input(&self, message_to_show: &str) -> FigureInput {
        println!("{}", message_to_show);

        let mut line = String::new();
        _ = std::io::stdin().read_line(&mut line).unwrap();

        let input: Vec<FigureColour> = line.trim_end().chars().map(|ch| {FigureColour::from_str(&ch.to_string()).unwrap()}).collect();
        println!("INPUT {:?}", input);

        FigureInput::new(&input)
    }
}

pub struct TerminalPresenter {

}

impl Visualizer for TerminalPresenter {
    fn present_game_state(&self, comparison: &HitOutput) {
        println!("{:?}", comparison);
    }
}
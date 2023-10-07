use algorithms::Algorithm;
use spinners::{Spinner, Spinners};
use std::{
    path::{Path, PathBuf},
    process::exit,
    time::Instant,
};

use clap::Parser;
use maze::Maze;

use crate::{
    algorithms::{
        a_star::AStar, breadth_first::BreadthFirst, dijkstra::Dijkstra, left_turn::LeftTurn, Solver,
    },
    img::Image,
};

mod algorithms;
mod img;
mod maze;
mod node;
mod utils;

const OUTPUT_FILENAME: &str = "./solution.png";
const LONG_DESC: &str = "

┌┬┐┬ ┬┌─┐┌─┐┌─┐┬ ┬┌─┐   ┬─┐┌─┐
 │ ├─┤├┤ └─┐├┤ │ │└─┐───├┬┘└─┐
 ┴ ┴ ┴└─┘└─┘└─┘└─┘└─┘   ┴└─└─┘

A small program that uses different algorithms to solve mazes.

Mazes need to be provided as raw uncompressed images with exactly one entrance at the top and one exit
at the bottom. The entire image needs to be surrounded by black borders and each wall and each path
needs to be exactly one pixel wide each.

there is currently no limit to how big a maze can be, but be wary of memory consumption, you have
been warned.";

const TITLE: &str = "

████████╗██╗  ██╗███████╗███████╗███████╗██╗   ██╗███████╗      ██████╗ ███████╗
╚══██╔══╝██║  ██║██╔════╝██╔════╝██╔════╝██║   ██║██╔════╝      ██╔══██╗██╔════╝
   ██║   ███████║█████╗  ███████╗█████╗  ██║   ██║███████╗█████╗██████╔╝███████╗
   ██║   ██╔══██║██╔══╝  ╚════██║██╔══╝  ██║   ██║╚════██║╚════╝██╔══██╗╚════██║
   ██║   ██║  ██║███████╗███████║███████╗╚██████╔╝███████║      ██║  ██║███████║
   ╚═╝   ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝ ╚═════╝ ╚══════╝      ╚═╝  ╚═╝╚══════╝
                                                                        
";

const SOLVED: &str = "

███╗   ███╗ █████╗ ███████╗███████╗    ███████╗ ██████╗ ██╗    ██╗   ██╗███████╗██████╗ 
████╗ ████║██╔══██╗╚══███╔╝██╔════╝    ██╔════╝██╔═══██╗██║    ██║   ██║██╔════╝██╔══██╗
██╔████╔██║███████║  ███╔╝ █████╗      ███████╗██║   ██║██║    ██║   ██║█████╗  ██║  ██║
██║╚██╔╝██║██╔══██║ ███╔╝  ██╔══╝      ╚════██║██║   ██║██║    ╚██╗ ██╔╝██╔══╝  ██║  ██║
██║ ╚═╝ ██║██║  ██║███████╗███████╗    ███████║╚██████╔╝███████╗╚████╔╝ ███████╗██████╔╝
╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝    ╚══════╝ ╚═════╝ ╚══════╝ ╚═══╝  ╚══════╝╚═════╝ 
                                                                                    
";

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Theseus-rs\n---------\nA small program that tries to solve mazes", 
    long_about = LONG_DESC,
)]
struct Cli {
    filename: Option<PathBuf>,

    #[arg(short, long, help = "Set output image filename")]
    output: Option<PathBuf>,

    #[arg(short, long, help = "Solve with Dijkstras algorithm")]
    dijkstra: bool,

    #[arg(short, long, help = "Solve with A* algorithm")]
    a_star: bool,

    #[arg(short, long, help = "Solve with always taking a left turn")]
    left_turn: bool,

    #[arg(short, long, help = "Solve with breadth first algorithm")]
    breadth_first: bool,
}

fn main() {
    let cli = Cli::parse();

    let filename = if let Some(filename) = cli.filename.as_deref() {
        filename
    } else {
        println!("No filename was provided");
        exit(1);
    };

    let algorithm = get_algorithm(&cli);

    if let Algorithm::None = algorithm {
        println!("No algorithm was defined, please check the help section (--help)");
        exit(1);
    }

    println!("{TITLE}");

    let mut spinner = Spinner::new(
        Spinners::Dots12,
        format!("loading image: {}", filename.display()),
    );
    let start = Instant::now();
    let mut image = Image::open(filename);
    spinner.stop_with_newline();
    let mut spinner = Spinner::new(Spinners::Dots12, "analyzing maze".into());
    let maze = Maze::from_image(&image);
    let load_duration = start.elapsed();
    spinner.stop_with_newline();
    let maze = maze.unwrap();
    println!(
        "loading maze: {} took: {:?}",
        filename.display(),
        load_duration
    );
    println!("number of nodes loaded: {}", maze.data.len());

    let solution_time = Instant::now();

    println!("Solution algorithm defined: {algorithm}");
    let mut spinner = Spinner::new(Spinners::Dots12, "lets solve this bad boy...".into());
    let result = match algorithm {
        Algorithm::LeftTurn => LeftTurn::solve(&maze),
        Algorithm::Dijkstra => Dijkstra::solve(&maze),
        Algorithm::AStar => AStar::solve(&maze),
        Algorithm::BreadthFirst => BreadthFirst::solve(&maze),
        _ => unreachable!(),
    };
    spinner.stop_with_newline();

    println!("{SOLVED}");
    let solution_time = solution_time.elapsed();
    println!("finding the solution took: {:?}", solution_time);

    let mut solution = result.unwrap();
    println!("number of decisions: {:?}", solution.count);

    image.apply_solution(&mut solution);

    let mut output_filename = OUTPUT_FILENAME;
    if let Some(output) = cli.output.as_deref() {
        output_filename = output.file_name().unwrap().to_str().unwrap();
        image.save(output).unwrap();
    } else {
        image.save(Path::new(OUTPUT_FILENAME)).unwrap();
    }
    println!("💾Saved solution to file: {}", output_filename);
    println!("Freeing up memory and exiting program.")
}

fn get_algorithm(cli: &Cli) -> Algorithm {
    if cli.dijkstra {
        Algorithm::Dijkstra
    } else if cli.a_star {
        Algorithm::AStar
    } else if cli.left_turn {
        Algorithm::LeftTurn
    } else if cli.breadth_first {
        Algorithm::BreadthFirst
    } else {
        Algorithm::None
    }
}

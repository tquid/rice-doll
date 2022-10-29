use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest/dice.pest"]
struct DiceParser;

pub fn print_pair(pair: &Pair<Rule>) {
    println!("Rule:       {:?}", pair.as_rule());
    println!("Span:       {:?}", pair.as_span());
    println!("Text:       {}", pair.as_str());
    println!("----------------");
}

pub fn parse() {
    let pair = DiceParser::parse(Rule::roll_command, "roll 3d6 4d10 1[* 2 3 4 5 +]")
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    print_pair(&pair);
    for inner_pair in pair.into_inner() {
        print_pair(&inner_pair);
    }
}
/*
pub enum Command {
    Roll,
}

pub enum Roll {
    Num(i32),
    DieDef(Vec<&str>),
    DieFace(&str),
    Modifier {
        filter: Filter,
        modifier: Modifier,
        num: Roll::Num,
    },
    Op,
}

pub enum Op {
    Reroll,
    Count,
    Explode,
}

pub enum Filter {
    LessThan,
    GreaterThan,
    Equal,
}
*/

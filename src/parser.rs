use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::process::exit;

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
    let pairs = DiceParser::parse(Rule::command, "roll 3d6=6# 100d10 1[* 2 3 4 5 +]")
        .unwrap_or_else(|e| {
            println!("{}", e);
            exit(0)
        });
    println!("{:}", pairs);
    for pair in pairs {
        println!("{}", pair)
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

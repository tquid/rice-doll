use crate::dice::Pool;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
    error::Error,
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

pub fn parse_command(text: &str) -> Result<Roll, Error<Rule>> {
    let pairs = DiceParser::parse(Rule::command, text)
        .unwrap_or_else(|e| {
            println!("{}", e);
            exit(0)
        });
    
    fn parse_value(pair: Pair<Rule>) -> Roll {
        match pair.as_rule() {
            // Don't care about command or roll_command yet, so just keep parsing
            Rule::command => pair.into_inner()
                .map(|pair| { parse_value(pair) }),
            Rule::roll_command => 
            Rule::dice => Pool.new(pair.into_inner()
                .map(|pair| {

                }))
        }
    }
    println!("{:}", pairs);
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

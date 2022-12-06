extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar="freight.pest"]
struct FreightParser;

use std::collections::VecDeque;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../../input/day_05.txt")?;

    //TODO:  Ugh what a messy parser clean up please !
    //TODO:  Add some tests !

    let freight_parser = FreightParser::parse(Rule::file, &input).unwrap().next().unwrap();
    let mut stacks: Vec<VecDeque<&str>> = vec![VecDeque::new(); 9];
    let mut instructions: Vec<(usize, usize, usize)> = Vec::new();

    let mut stack_number = 1;
    for line in freight_parser.into_inner() {
        match line.as_rule() {
            Rule::container_list => {
                for rule in line.into_inner() {
                    match rule.as_rule() {
                        Rule::empty => stack_number += 1,
                        Rule::cname => {
                            stacks[stack_number - 1].push_back(rule.as_str());
                            stack_number += 1;
                        },
                        Rule::linebreak => stack_number = 1,
                        _ => unreachable!(),
                    }
                }

            }
            Rule::stack_number => continue, //NOTE: Parsed this but do not need it at all...
            Rule::instructions => {
                let mut inner_rules = line.into_inner();
                inner_rules.next();
                let amount = inner_rules.next().unwrap().as_str().parse::<usize>().unwrap();
                inner_rules.next();
                let stack_from = inner_rules.next().unwrap().as_str().parse::<usize>().unwrap() - 1;
                inner_rules.next();
                let stack_to = inner_rules.next().unwrap().as_str().parse::<usize>().unwrap() - 1;

                instructions.push((amount, stack_from, stack_to));
            },
            Rule::EOI => break,
            _ => unreachable!(),
        }
    }

    part_a(&mut stacks.clone(), &instructions);
    part_b(&mut stacks.clone(), &instructions);

    Ok(())
}

fn part_a(stacks: &mut Vec<VecDeque<&str>>, instructions: &[(usize,usize,usize)]) {
    for instruction in instructions.iter() {
        //println!("{id}, {:?}", instruction);

        for _ in 0..instruction.0 {
            //println!("container gets moved!");
            let container = stacks[instruction.1].pop_front().unwrap();
            stacks[instruction.2].push_front(container);
        }
        //println!("Stacks: {:?}", stacks);
    }

    println!("The standard cranelift priduces the follwing result");
    for stack in stacks {
        if stack.len() >= 1 {
            print!("{}", stack[0]);
        }
    }
    println!();
}

fn part_b(stacks: &mut Vec<VecDeque<&str>>, instructions: &[(usize,usize,usize)]) {
    for instruction in instructions.iter() {
        let elements = stacks[instruction.1]
                .drain(0..instruction.0)
                .collect::<VecDeque<_>>();
            //println!("Took {} from stack {} and push it onto {}", container, instruction.1, instruction.2);

        for index in (0..elements.len()).rev() {
            stacks[instruction.2].push_front(elements[index]);
        }
    }

    print!("The cranelift 9001 produces the follwing result: ");
    for stack in stacks {
        if stack.len() >= 1 {
            print!("{}", stack[0]);
        }
    }
    println!();
}

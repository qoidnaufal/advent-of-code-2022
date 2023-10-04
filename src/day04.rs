#[allow(unused_imports)]
use std::collections::{HashSet, HashMap};

#[allow(dead_code)]
static INPUT: &'static str = include_str!("./day04.txt");

#[derive(Debug, Clone)]
struct Assignment {
    #[allow(dead_code)]
    section1: Vec<u64>,
    #[allow(dead_code)]
    section2: Vec<u64>
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<Assignment> {
    input.lines()
        .map(|chr| {
            chr.split(",").collect::<Vec<_>>()
        })
        .map(|ghj| {

            let a = ghj[0]
                .split("-")
                .map(|chr| {
                    chr.parse::<u64>().ok().unwrap()
                })
                .collect::<Vec<_>>();

            let b = ghj[1]
                .split("-")
                .map(|chr| {
                    chr.parse::<u64>().ok().unwrap()
                })
                .collect::<Vec<_>>();

            Assignment {
                section1: a,
                section2: b
            }
        }).collect()
}

#[test]
fn part1() {
    let a = parse_input(INPUT);

    let b = a.iter().filter(|stc| {

        let range1 = (stc.section1[0]..=stc.section1[1]).collect::<HashSet<_>>();
        let range2 = (stc.section2[0]..=stc.section2[1]).collect::<HashSet<_>>();

        range1.is_subset(&range2) || range1.is_superset(&range2)
    })
    .collect::<Vec<_>>();

    println!("{:?}", b.len());
}

#[test]
fn part2() {
    let a = parse_input(INPUT);

    let b = a.iter().filter(|elem| {
        
        let range1 = (elem.section1[0]..=elem.section1[1]).collect::<HashSet<_>>();
        let range2 = (elem.section2[0]..=elem.section2[1]).collect::<HashSet<_>>();

        !range1.is_disjoint(&range2)
    })
    .collect::<Vec<_>>();

    println!("{}", b.len());
}
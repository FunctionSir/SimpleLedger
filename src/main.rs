/*
 * @Author: FunctionSir
 * @License: AGPLv3
 * @Date: 2023-12-11 22:53:52
 * @LastEditTime: 2023-12-12 20:45:29
 * @LastEditors: FunctionSir
 * @Description: -
 * @FilePath: /SimpleLedger/src/main.rs
 */
extern crate ini;
use ini::Ini;
use std::io::{self, stdout, Write};
fn disp_record(ledger: &Ini, record: &str) {
    println!("Record No. {}", record);
    let section = ledger.section(Some(record)).unwrap();
    println!("Name: {}", section.get("Name").unwrap());
    println!("Description: {}", section.get("Desc").unwrap());
    println!("Tags: {}", section.get("Tags").unwrap());
    let change_cent: i64 = section.get("Cash").unwrap().parse().unwrap();
    let change_frac: i64 = change_cent.abs() % 100;
    let change_round: i64 = (change_cent.abs() - change_frac) / 100;
    println!(
        "{}: {}.{:02}",
        if change_cent >= 0 { "Income" } else { "Expend" },
        change_round,
        change_frac
    );
    stdout().flush().unwrap();
}
fn calculate(ledger: &Ini) {
    let mut total_income_cent: i64 = 0;
    let mut total_expend_cent: i64 = 0;
    for i in 0..ledger.sections().count() - 1 {
        let section = ledger.section(Some(i.to_string())).unwrap();
        let change_cent: i64 = section.get("Cash").unwrap().parse().unwrap();
        if change_cent >= 0 {
            total_income_cent += change_cent;
        } else {
            total_expend_cent += change_cent.abs();
        }
    }
    let total_income_frac: i64 = total_income_cent % 100;
    let total_expend_frac: i64 = total_expend_cent % 100;
    let total_income_round: i64 = (total_income_cent - total_income_frac) / 100;
    let total_expend_round: i64 = (total_expend_cent - total_expend_frac) / 100;
    println!(
        "Total Income\t{}.{:02}",
        total_income_round, total_income_frac
    );
    println!(
        "Total Expend\t{}.{:02}",
        total_expend_round, total_expend_frac
    );
    let total_cent: i64 = total_income_cent - total_expend_cent;
    let total_frac: i64 = total_cent.abs() % 100;
    let total_round: i64 = total_cent + (if total_cent > 0 { -1 } else { 1 }) * total_frac;
    println!("TOTAL\t\t{}.{:02}", total_round, total_frac);
    stdout().flush().unwrap();
}
fn main() {
    println!("Simple Ledger Ver 0.1.0 (Furina)");
    println!("This is a libre software under AGPLv3");
    print!("(LOAD) ");
    stdout().flush().unwrap();
    let mut ledger_file = String::new();
    io::stdin()
        .read_line(&mut ledger_file)
        .expect("Failed to read line!");
    let ledger_file = ledger_file.trim();
    let ledger = Ini::load_from_file(ledger_file).unwrap();
    println!("Loaded Ledger: {ledger_file}");
    println!("Record Count: {}", ledger.sections().count() - 1);
    loop {
        print!("(V/C) ");
        stdout().flush().unwrap();
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line!");
        let cmd = cmd.trim();
        let op: Vec<&str> = cmd.split(':').collect();
        if op[0].to_uppercase() == "V" {
            disp_record(&ledger, &op[1]);
        } else if op[0].to_uppercase() == "C" {
            calculate(&ledger);
        }
    }
}

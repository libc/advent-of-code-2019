use std::collections::HashMap;
use std::collections::HashSet;

struct Exchanger {
    rates: HashMap<String, ExchangeRate>,
}

struct ExchangeRate {
    amount: usize,
    inputs: Vec<(usize, String)>,
}

impl Exchanger {
    fn parse(input: &str) -> Exchanger {
        let rates = input.trim().split("\n").map(|line| {
            let mut v = line.trim().split("=>");

            let rates = v
                .nth(0)
                .unwrap()
                .trim()
                .split(",")
                .map(|p| Exchanger::parse_pair(p))
                .collect();

            let (a, s) = Exchanger::parse_pair(&v.nth(0).unwrap());

            (
                s,
                ExchangeRate {
                    amount: a,
                    inputs: rates,
                },
            )
        });

        Exchanger {
            rates: rates.collect(),
        }
    }

    fn parse_pair(input: &str) -> (usize, String) {
        let mut v = input.trim().split(" ");

        let a = v.nth(0).unwrap().parse::<usize>().unwrap();
        let s = String::from(v.nth(0).unwrap());

        (a, s)
    }

    fn exchange(&self, want_amount: usize, want_currency: &str, got_currency: &str) -> usize {
        let mut wallet = HashMap::new();

        wallet.insert(String::from(want_currency), want_amount);

        let currencies = self.tsort();
        let mut idx = 0;

        while idx < currencies.len() {
            let currency = currencies[idx].clone();
            if currency == got_currency {
                break;
            }
            let amount = *wallet.get(&currency).unwrap_or(&0);
            if amount == 0 {
                idx += 1;
                continue;
            }

            let rate = self.rates.get(&currency).unwrap();

            let m = if rate.amount < amount {
                amount / rate.amount
            } else {
                1
            };

            for (a, s) in rate.inputs.iter() {
                let v = wallet.entry(String::from(s)).or_insert(0);
                *v += a * m;
            }

            if rate.amount < amount {
                let v = wallet.entry(currency).or_insert(0);
                *v -= rate.amount * m;
            } else {
                wallet.remove(&String::from(currency));
            }
        }

        *wallet.get(got_currency).unwrap()
    }

    fn find(&self, want: &str, given_amount: usize, given_currency: &str) -> usize {
        let mut left = 1;
        let mut right = given_amount;

        while (right - left) > 100 {
            let midpoint = left + (right - left) / 2;
            println!("{} {} {}", left, midpoint, right);

            if self.exchange(midpoint, want, given_currency) > given_amount {
                right = midpoint;
            } else {
                left = midpoint;
            }
        }

        for i in left..=right {
            if self.exchange(i + 1, want, given_currency) > given_amount {
                return i;
            }
        }

        left
    }

    fn tsort(&self) -> Vec<String> {
        let mut output = Vec::new();

        let mut permanent_mark = HashSet::new();
        let mut temporary_mark = HashSet::new();

        fn visit(
            rates: &HashMap<String, ExchangeRate>,
            permanent_mark: &mut HashSet<String>,
            temporary_mark: &mut HashSet<String>,
            output: &mut Vec<String>,
            n: &String,
        ) {
            if permanent_mark.contains(n) {
                return;
            }
            if temporary_mark.contains(n) {
                panic!("not a DAG!!");
            }

            temporary_mark.insert(String::from(n));

            let inputs = rates.get(n);
            if inputs.is_some() {
                for (_, m) in inputs.unwrap().inputs.iter() {
                    visit(rates, permanent_mark, temporary_mark, output, m);
                }
            }
            temporary_mark.remove(n);

            permanent_mark.insert(String::from(n));

            output.push(String::from(n));
        }

        loop {
            let mut node = None;

            for (k, _) in &self.rates {
                if !permanent_mark.contains(k) {
                    node = Some(String::from(k));
                    break;
                }
            }

            if node.is_none() {
                break;
            }

            visit(
                &self.rates,
                &mut permanent_mark,
                &mut temporary_mark,
                &mut output,
                &node.unwrap(),
            );
        }

        output.into_iter().rev().collect()
    }
}

fn main() {
    let input1 = r#"
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL
        "#;

    println!("{}", Exchanger::parse(&input1).exchange(1, "FUEL", "ORE"));

    let input2 = r#"
        157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
        "#;
    println!("{}", Exchanger::parse(&input2).exchange(1, "FUEL", "ORE"));
    println!(
        "{}",
        Exchanger::parse(&input2).find("FUEL", 1000000000000, "ORE")
    );

    let input3 = r#"
        171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX
        "#;
    println!("{}", Exchanger::parse(&input3).exchange(1, "FUEL", "ORE"));
    println!(
        "{}",
        Exchanger::parse(&input3).find("FUEL", 1000000000000, "ORE")
    );

    let task = r#"
        4 NZGF => 6 WBMZG
        20 FWMN, 2 QTMF, 5 FMVDV, 1 CVBPJ, 2 KVJK, 20 XSTBX, 7 NBFS => 5 SHPSF
        7 LVQM => 5 NXDHX
        1 FNDMP, 1 QZJV, 12 RMTG => 7 JBFW
        10 GKVF, 1 NXDHX => 8 NZGF
        12 QZJV => 8 RSMC
        8 RWTD => 7 NBFS
        4 CZGXS, 25 QTMF, 2 PHFQB => 3 BWQN
        3 WQZD => 9 CTZKV
        2 DCTQ, 18 CTZKV => 4 QLHZW
        31 QLHZW, 11 FNDMP => 6 WFDXN
        8 RLQC => 2 ZPJS
        2 SWSQG, 13 CVBPJ => 9 DWCND
        7 PBXB, 6 HKSWM, 4 BDPC, 4 KVJK, 2 ZLGKH, 9 LXFG, 1 ZPJS => 4 SWCWH
        6 QZJV => 7 RLQC
        3 QZJV, 11 MRQHX, 15 GKVF => 4 FMVDV
        3 NXDHX, 1 GKNQL => 3 VMDS
        1 VMDS => 2 RHSQ
        13 GKNQL, 4 NXDHX, 2 GKVF => 8 MRQHX
        4 PVRN => 2 WBSL
        2 CVBPJ => 9 PVRN
        3 FNDMP => 9 BZKC
        180 ORE => 6 FWMN
        13 DCTQ, 2 RHSQ => 5 CVBPJ
        1 DWCND, 12 BZKC, 2 WBRBV => 6 HTLZ
        1 LMGL, 11 XDVL, 7 DWCND => 5 ZLGKH
        3 FMFTD => 3 HKSWM
        1 FNDMP, 5 RMTG, 3 QLHZW => 9 CZGXS
        7 DCTQ => 3 FNDMP
        1 SHPSF, 2 SWCWH, 40 WFDXN, 67 WBMZG, 53 WBSL, 2 CQJDJ, 41 BWQN, 12 GMQVW, 48 PDRJ, 42 RSMC => 1 FUEL
        3 VMDS, 1 BHRZ => 9 DCTQ
        22 DCTQ, 4 NZGF => 7 RMTG
        29 RWTD, 3 FMFTD => 5 LMGL
        12 WBRBV, 13 PDRJ, 36 RSRG => 4 LXFG
        1 SWSQG, 2 NLPB => 3 WBRBV
        7 HTKLM, 8 CTZKV => 2 RWTD
        4 BQXL, 1 FWMN => 9 GKNQL
        4 WFDXN => 9 HTKLM
        2 XDVL => 5 QTMF
        1 PHFQB, 21 LMGL, 2 SWSQG => 7 GMQVW
        23 CZGXS, 11 FMVDV => 3 PDRJ
        1 DWCND, 1 NPMXR, 1 RSRG, 1 JBFW, 12 VXWKZ, 9 KVJK => 4 CQJDJ
        106 ORE => 4 BQXL
        4 PHFQB => 8 NPMXR
        1 GKNQL => 8 WQZD
        6 BDPC => 2 PHFQB
        1 DWCND => 7 PBXB
        1 RSMC, 1 PDRJ => 8 SWSQG
        1 LVQM => 4 BHRZ
        7 CVBPJ, 1 SWSQG, 1 NLPB => 2 VXWKZ
        1 BHRZ, 1 JBFW => 6 XDVL
        12 LMGL, 8 RWTD => 4 XSTBX
        4 RSMC => 6 BDPC
        1 BHRZ, 5 NXDHX => 3 GKVF
        6 FMVDV, 6 VXWKZ, 37 CVBPJ => 5 KVJK
        7 NLPB, 3 HTLZ => 4 RSRG
        1 PDRJ => 1 FMFTD
        6 RHSQ, 1 NZGF => 5 QZJV
        127 ORE => 3 LVQM
        3 RHSQ, 2 RLQC, 1 WFDXN => 1 NLPB
        "#;
    println!("{}", Exchanger::parse(&task).exchange(1, "FUEL", "ORE"));
    println!(
        "{}",
        Exchanger::parse(&task).find("FUEL", 1000000000000, "ORE")
    );
}

fn parse_lines(input: &str) -> Vec<String> {
    let mut v = Vec::new();
    let mut s = String::new();
    for line in input.split("\n") {
        if line.trim().len() == 0 {
            v.push(s);
            s = String::new();
        }
        else {
            s.push_str(line);
            s.push_str("\n");
        }
    }
    v.push(s);
    v
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    List(Vec<Value>),
    Integer(u8)
}

fn parse_list(mut line: &str) -> Vec<Value> {
    let mut vals = Vec::new();
    while !line.is_empty() {
        vals.push(
            if &line[0..1] == "[" {
                let mut level = 1;
                let mut idx = 1;
                while idx < line.len() && !(&line[idx.. idx + 1] == "," && level == 0) {
                    if &line[idx..idx + 1] == "[" {
                        level += 1;
                    } 
                    else if &line[idx..idx + 1] == "]" {
                        level -= 1;
                    }
                    idx += 1;
                }
                let new_val = parse_list(&line[1..idx - 1]);
                let min = if line.len() < idx + 1 { line.len() } else { idx + 1 };
                line = &line[min .. line.len()]; // skip comma
                Value::List(new_val)
            } 
            else {
                let end_idx = line.find(",").or(Some(line.len())).unwrap();
                let num = line[0..end_idx].parse().unwrap();

                let min = if line.len() < end_idx + 1 { line.len() } else { end_idx + 1 };
                line = &line[min..line.len()]; // Skip comma
                Value::Integer(num)
            }
        );
    }
    vals
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Integer(i1), Value::Integer(i2)) => {
                if i1 < i2 {
                    std::cmp::Ordering::Less
                }
                else if i1 > i2 {
                    std::cmp::Ordering::Greater
                }
                else  {
                    std::cmp::Ordering::Equal
                }
            },
            (Value::List(l1), Value::List(l2)) => {
                for idx in 0..l1.len() {
                    if l2.len() <= idx {
                        break;
                    }
                    if l1[idx].cmp(&l2[idx]) != std::cmp::Ordering::Equal {
                        return l1[idx].cmp(&l2[idx]);
                    }
                }
                if l1.len() != l2.len() {
                    if l1.len() < l2.len() {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                } 
                else {
                    std::cmp::Ordering::Equal
                }   
            }
            (Value::List(_), Value::Integer(i)) => {
                self.cmp(&Value::List(vec![Value::Integer(*i)]))
            }
            (Value::Integer(i), Value::List(_)) => {
                Value::List(vec![Value::Integer(*i)]).cmp(other)
            }
        }
    }
}

pub fn part1(input: &str) {
    let mut packets = Vec::new();
    for lines in parse_lines(input){
        let [line1, line2] = lines.split("\n").next_chunk().unwrap();
        let line1 = &line1[1..line1.len() - 1];
        let line2 = &line2[1..line2.len() - 1];
        packets.push((Value::List(parse_list(line1)), Value::List(parse_list(line2))));
    }
    let mut sum = 0;
    for idx in 0..packets.len() {
        let (pack1, pack2) = &packets[idx];
        if pack1 < pack2 {
            sum += idx + 1
        }
    }
    println!("{}", sum);
}

pub fn part2(input: &str) {
    let mut packets = Vec::new();
    for line in input.split("\n") {
        if !line.trim().is_empty() {
            packets.push(Value::List(parse_list(line)));
        }
    }
    // divider packets
    let div_pkt_1 = Value::List(vec![Value::List(vec![Value::Integer(2)])]);
    let div_pkt_2 = Value::List(vec![Value::List(vec![Value::Integer(6)])]);
    packets.push(div_pkt_1.clone());
    packets.push(div_pkt_2.clone());

    packets.sort();
    let div_idx_1 = packets.iter().position(|v| *v == div_pkt_1).unwrap() + 1;
    let div_idx_2 = packets.iter().position(|v| *v == div_pkt_2).unwrap() + 1;

    println!("{}", div_idx_1 * div_idx_2);
}

pub fn day13(input: &str) {
    part1(input);
    part2(input);
}
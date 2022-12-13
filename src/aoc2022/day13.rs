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

fn compare_values(val1: &Value, val2: &Value) -> Option<bool> {
    //println!("Comparing {:?}, {:?}", val1, val2);
    match (val1, val2) {
        (Value::Integer(i1), Value::Integer(i2)) => {
            if i1 < i2 {
                Some(true)
            }
            else if i1 > i2 {
                Some(false)
            }
            else  {
                None
            }
        },
        (Value::List(l1), Value::List(l2)) => {
            for idx in 0..l1.len() {
                if l2.len() <= idx {
                    break;
                }
                if let Some(res) = compare_values(&l1[idx], &l2[idx]) {
                    return Some(res);
                }
            }
            if l1.len() != l2.len() {
                Some(l1.len() < l2.len())
            } 
            else {
                None
            }   
        }
        (Value::List(_), Value::Integer(i)) => {
            compare_values(val1, &Value::List(vec![Value::Integer(*i)]))
        }
        (Value::Integer(i), Value::List(_)) => {
            compare_values(&Value::List(vec![Value::Integer(*i)]), val2)
        }
    }
}

fn try_order(front_val: &Value, rest: &Vec<Value>) -> Option<Vec<Value>> {
    for val_idx in 0..rest.len() {
        if compare_values(front_val, &rest[val_idx]).unwrap() {
            let mut new_rest = (*rest).clone();
            let new_front = new_rest.remove(val_idx);
            if let Some(mut rest) = try_order(&new_front, &new_rest) {
                let mut new_vec = vec![front_val.clone()];
                new_vec.append(&mut rest);
                return Some(new_vec)
            } 
        }
    }
    None
}

fn flatten(values: &[Value]) -> Option<u8> {
    if values.len() == 0 {
        None
    }
    else {
        match &values[0] {
            Value::Integer(i) => Some(*i),
            Value::List(v) => flatten(v)
        }
    }
}

fn flat_compare(first: &[Value], second: &[Value]) -> Option<bool> {
    assert_ne!(first.len(), 0);
    assert_ne!(second.len(), 0);
    
    let first_val = flatten(first);
    let second_val = flatten(second);

    if first_val == second_val {
        None
    } 
    else if first_val.is_none() {
        Some(false)
    } 
    else if second_val.is_none() {
        Some(true)
    } 
    else {
        Some(first_val.unwrap() < second_val.unwrap())
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut this = self.clone();
        let mut other = other.clone();
        if matches!(this, Value::Integer(_)) {
            this = Value::List(vec![this.clone()]);
        }
        if matches!(other, Value::Integer(_)) {
            other = Value::List(vec![other.clone()]);
        }
        match (&this, &other) {
            (Value::List(l1), Value::List(l2)) => {
                let mut l1 = l1.as_slice();
                let mut l2 = l2.as_slice();
                while l1.len() > 0 && l2.len() > 0 {
                    let cmp = if matches!(l1[0], Value::List(_)) || matches!(l2[0], Value::List(_)) 
                    {
                        l1[0].cmp(&l2[0])
                    }
                    else if let Value::Integer(i1) = l1[0] && let Value::Integer(i2) = l2[0] {
                        i1.cmp(&i2)
                    }
                    else {
                        unreachable!();
                    };
                    if cmp != std::cmp::Ordering::Equal {
                        return cmp;
                    }
                    l1 = &l1[1..l1.len()];
                    l2 = &l2[1..l2.len()];
                }
                if l2.len() > 0 {
                    return std::cmp::Ordering::Less;
                } 
                else if l1.len() > 0 {
                    return std::cmp::Ordering::Greater;
                }
            },
            _ => unreachable!()
        }
        //println!("{:?}", this);
        //println!("{:?}", other);
        //unreachable!()
        std::cmp::Ordering::Equal
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
        if let Some(res) = compare_values(&pack1, &pack2) {
            //println!("Pair {} is correct? {}", idx + 1, res);
            //println!("p1: {:?}", pack1);
            //println!("p2: {:?}", pack2);

            if res {
                sum += idx + 1;
            }
        }
        else {
            println!("Two packets as lists were equal!");
            println!("p1: {:?}", pack1);
            println!("p2: {:?}", pack2);
            panic!();
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
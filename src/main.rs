use std::collections::HashMap;
use std::fs as fileRead;
use std::io::Error;

fn get_target_binary(target: &str) -> String {
    if target == "D" {
        return "010".to_string();
    } else {
        return "100".to_string();
    }
}

fn get_jump_binary(jump: &str) -> String {
    let binary_jump_hashmap: HashMap<&str, &str> = HashMap::from([
        ("JGT", "001"),
        ("JEQ", "010"),
        ("JGE", "011"),
        ("JLT", "100"),
        ("JNE", "101"),
        ("JLE", "110"),
    ]);

    return binary_jump_hashmap.get(jump).unwrap_or(&"").to_string();
}

fn get_src_binary(src: &str) -> String {
    let binary_src_hashmap: HashMap<&str, &str> = HashMap::from([
        ("0", "101010"),
        ("1", "111111"),
        ("-1", "111010"),
        ("D", "001100"),
        ("A", "110000"),
        ("(A)", "110000"),
    ]);

    return binary_src_hashmap.get(src).unwrap_or(&"").to_string();
}

fn get_access_binary(access: &str) -> String {
    if access == "(A)" {
        return "1".to_string();
    } else {
        return "0".to_string();
    }
}

fn read_file_in(filename: &str) -> Result<Vec<String>, Error> {
    let contents: String = fileRead::read_to_string(filename)?;
    let lines: Vec<String> = contents
        .lines()
        .map(|line| line.trim().to_string().replace(" ", "").to_uppercase())
        .collect();
    Ok(lines)
}

fn go_through_lines(lines: Vec<String>) {
    let mut list_of_instructions: Vec<String> = Vec::new();

    for line in lines {
        println!("{}", line);

        if line.trim() == "" {
            continue;
        }

        let binary_instruction: String;

        let instruction: String = line.chars().take(3).collect();
        let rest: String = line.chars().skip(3).collect();

        match instruction.as_str() {
            "JMP" => binary_instruction = "1110101010000111".to_string(),
            "ADD" => binary_instruction = add_function(&rest),
            "SUB" => binary_instruction = sub_function(&rest),
            "JGT" | "JEQ" | "JGE" | "JLT" | "JNE" | "JLE" => {
                binary_instruction = jump_instruction(&instruction, &rest)
            }
            "STR" => binary_instruction = str_function(&rest),
            "LDR" => binary_instruction = ldr_function(&rest),
            _ => continue,
        }

        // println!("{}", binary_instruction);

        list_of_instructions.push(binary_instruction);
    }

    for instruction in list_of_instructions {
        println!("{}", instruction);
    }
}

fn add_function(rest: &str) -> String {
    println!("{}", rest);

    let split_chars: Vec<&str> = rest.split(",").collect();

    let target: &str = split_chars[0];
    let access: &str = split_chars[2];

    let target_binary: &str = &get_target_binary(target);
    let access_binary: &str = &get_access_binary(access);

    let combined_add_binary: String = format!("111{}000010{}000", access_binary, target_binary);

    return combined_add_binary;
}

fn sub_function(rest: &str) -> String {
    println!("{}", rest);

    let split_chars: Vec<&str> = rest.split(",").collect();

    let target: &str = split_chars[0];
    let access: &str = split_chars[2];

    let target_binary: &str = &get_target_binary(target);
    let access_binary: &str = &get_access_binary(access);

    let combined_sub_binary: String = format!("111{}010011{}000", target_binary, access_binary);

    return combined_sub_binary;
}

fn jump_instruction(instruction: &str, rest: &str) -> String {
    let jump_binary: &str = &get_jump_binary(instruction);
    let access_binary: &str = &get_access_binary(rest);
    let src_binary: &str = &get_src_binary(rest);

    let combined_jump_binary: String =
        format!("111{}{}000{}", access_binary, src_binary, jump_binary);

    return combined_jump_binary;
}

fn str_function(rest: &str) -> String {
    let split_chars: Vec<&str> = rest.split(",").collect();

    let access_binary: &str = &get_access_binary(split_chars[0]);
    let src_binary: &str = &get_src_binary(split_chars[1]);

    let combined_str_binary: String = format!("111{}{}001000", access_binary, src_binary);

    return combined_str_binary;
}

fn ldr_function(rest: &str) -> String {
    let split_chars: Vec<&str> = rest.split(",").collect();

    if split_chars[0] == "A" && split_chars[1].chars().next() == Some('$') {
        return ldr_number_binary(split_chars[1]);
    } else {
        return ldr_source_value(split_chars[0], split_chars[1]);
    }
}

fn ldr_number_binary(binary_number: &str) -> String {
    let number_string: String = binary_number.chars().skip(1).collect();
    let number: i32 = number_string.parse().unwrap();
    let binary_number: String = format!("{:016b}", number);
    return binary_number;
}

fn ldr_source_value(target: &str, src: &str) -> String {
    let access_binary: String = get_access_binary(src);
    let src_bianry: String = get_src_binary(src);
    let target_binary: String = get_target_binary(target);

    let combined_binary: String = format!("111{}{}{}000", access_binary, src_bianry, target_binary);

    return combined_binary;
}

fn main() {
    let filename: &str = "./testcases/test1.nha";

    let lines: Vec<String> = match read_file_in(filename) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            return;
        }
    };

    go_through_lines(lines);
}

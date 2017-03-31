use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main(){
	// testing();

	let fname = prompt_for_string();

	let file = match File::open(&fname) {
		Err(err) => panic!("File {} not found; error {}", &fname, err),
		Ok(file) => file,
	};
	let buf_reader = BufReader::new(&file);

	let mut linevec: Vec<String> = vec![];

	for line in buf_reader.lines() {
		let line_val = line.unwrap();
		// println!("{}", &line_val);
		linevec.push(line_val);
	}

	let mut resvec: Vec<String> = linevec
		.iter()
		.map(|x| assemble(x.to_string()))
		.collect::<Vec<String>>();

	println!("\nProgram is:");

	for res in &mut resvec {
		
		let resparts: Vec<&str> = res.split(|c| c == ':').collect();

		if resparts[0] != "NotInst".to_string() {
			println!("\t{}\t{}", resparts[0], resparts[1]);
		}
	}

	let path = Path::new("program.txt");
	let display = path.display();
	let mut outfile = match File::create(&path) {
		Err(why) => panic!("Couldn't create {}.", display),
		Ok(file) => file,
	};

	for res in &mut resvec {
		let resparts: Vec<&str> = res.split(|c| c == ':').collect();
		if resparts[0] != "NotInst".to_string() {
			outfile.write_all(resparts[0].as_bytes()).expect("Unable to write data!");
			outfile.write_all("\n".as_bytes()).expect("Unable to write data!");
		}
	}

}

fn prompt_for_string() -> String {
	let mut strbuf = String::new();
	let mut stdout = io::stdout();
	let stdin = io::stdin();

	print!("Enter a filename: ");
	let _ = stdout.flush();
	let mut handle = stdin.lock();
	let _ = handle.read_line(&mut strbuf);
	strbuf.pop();
	// strbuf.pop();
	strbuf
}

fn assemble(line: String) -> String {
	let mut finalstr = String::new();
	let v: Vec<&str> = line.split(|c| c == ' ' || c == ',' || c == '\t' || c == ';').collect();
	let mut cleanvec: Vec<String> = vec![];
	
	for item in v {
		if !item.is_empty() && item != "\t" {
			cleanvec.push(item.to_string());
		}
	}

	if cleanvec.len() == 0 {
		return "NotInst".to_string();
	}

	finalstr = match cleanvec[0].as_ref() {
		"NOP" 	=> String::from("0000000000000000"),
		"LD"		=> memop(&cleanvec),
		"ST" 		=> memop(&cleanvec),
		"MOV" 	=>	mov_arith(&cleanvec),
		"LIL" 	=> loadimm(&cleanvec),
		"LIH" 	=> loadimm(&cleanvec),
		"ADD"		=> mov_arith(&cleanvec),
		"ADC"		=> mov_arith(&cleanvec),
		"SUB"		=> mov_arith(&cleanvec),
		"SBC"		=> mov_arith(&cleanvec),
		"AND"		=> mov_arith(&cleanvec),
		"OR"		=> mov_arith(&cleanvec),
		"XOR"		=> mov_arith(&cleanvec),
		"NOT"		=> mov_arith(&cleanvec),
		"SL"		=>	mov_arith(&cleanvec),
		"SRL"		=> mov_arith(&cleanvec),
		"SRA"		=> mov_arith(&cleanvec), 
		"RRA"		=> mov_arith(&cleanvec),
		"RR"		=> mov_arith(&cleanvec),
		"RL"		=> mov_arith(&cleanvec),
		"JMP"		=> jmp(&cleanvec),
		"JAL" 	=> jmp(&cleanvec),
		"BR"		=> branch(&cleanvec),
		"BC"		=> branch(&cleanvec),
		"BO"		=> branch(&cleanvec),
		"BN"		=> branch(&cleanvec),
		"BZ"		=> branch(&cleanvec),
		_ 			=> String::from("NotInst"),
	} ;
	let delim = ":";
	let finalstr_ret = finalstr + &delim + &line;
	finalstr_ret
}

fn memop(lvec: &Vec<String>) -> String {
	let opcode = match lvec[0].as_ref() {
		"LD" => "001",
		"ST" => "010",
		_ => panic!("Invalid opcode."),
	}.to_string();
	let destid = xlate_register(&lvec[1]);
	let baseid = xlate_register(&lvec[2]);
	let offset = hex_to_bin(&lvec[3], 5);

	let machine_inst = opcode + &destid + &baseid + &offset;
	machine_inst
}

fn loadimm(lvec: &Vec<String>) -> String {
	let opcode = "100".to_string();
	let destid = xlate_register(&lvec[1]);
	let bit8 = match lvec[0].as_ref() {
		"LIL" => "0",
		"LIH" => "1",
		_ => panic!("Invalid instruction detected down call chain."),
	};
	let offset = hex_to_bin(&lvec[2], 8);

	let machine_inst = opcode + &destid + &bit8 + &offset;
	machine_inst
}

fn mov_arith(lvec: &Vec<String>) -> String {
	let opcode = "101".to_string();
	let destid = xlate_register(&lvec[1]);
	let srcid = xlate_register(&lvec[2]);

	let arithopid = match lvec[0].as_ref() {
		"ADD"		=> "00000",
		"ADC"		=> "00001",
		"SUB"		=> "00010",
		"SBC"		=> "00011",
		"AND"		=> "00100",
		"OR"		=> "00101",
		"XOR"		=> "00110",
		"NOT"		=> "00111",
		"SL"		=>	"01000",
		"SRL"		=> "01001",
		"SRA"		=> "01010", 
		"RRA"		=> "01110",
		"RR"		=> "01101",
		"RL"		=> "01100",
		_ 			=> panic!("Invalid instruction {}", lvec[0]),
	} ;
	let machine_inst = opcode + &destid + &srcid + &arithopid;
	machine_inst
}

fn jmp(lvec: &Vec<String>) -> String {
	let opcode = "110".to_string();
	let destid = xlate_register(&lvec[1]);
	let bit8 = "0".to_string();
	let offset = hex_to_bin(&lvec[2], 8);

	let machine_inst = opcode + &destid + &bit8 + &offset;
	machine_inst
}

fn branch(lvec: &Vec<String>) -> String {
	let opcode = "111".to_string();
	let mask = match lvec[0].as_ref() {
		"BR" => "0000",
		"BC" => "1000",
		"BO" => "0100",
		"BN" => "0010",
		"BZ" => "0001",
		_ => panic!("Invalid instruction {}", lvec[0]),
	};
	let bit8 = "0".to_string();
	let offset = hex_to_bin(&lvec[1],8);
	let machine_inst = opcode + &mask + &bit8 + &offset;
	machine_inst
}


fn xlate_register(rstring : &String) -> String {
	let result = match rstring.as_ref() {
		"R0" => "0000",
		"R1" => "0001",
		"R2" => "0010",
		"R3" => "0011",
		"R4" => "0100",
		"R5" => "0101",
		"R6" => "0110",
		"R7" => "0111",
		"R8" => "1000",
		"R9" => "1001",
		"R10" => "1010",
		"R11" => "1011",
		"R12" => "1100",
		"R13" => "1101",
		"R14" => "1110",
		"R15" => "1111",
			_  => panic!("Invalid register name {}", rstring),
	};
	// println!("Register: {}", result);
	result.to_string()
}

fn hex_to_bin(hexin : &String, width : usize) -> String {

    let mut outstr = String::new();
    let mut bflag = false;
    
    for c in hexin.chars() {
        if c == 'x'{
            bflag = true;
            continue;
        }
        if bflag{
            let binrep = match c {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                _   => panic!("'{}' not a valid hex character!", c),
            };
            outstr = outstr + &binrep;
        }
    }
    let mut outstr_rev: String = outstr
        .chars()
        .rev()
        .collect();
    outstr_rev.truncate(width);
    let finalstr: String = outstr_rev
        .chars()
        .rev()
        .collect();
    // println!("Offset: {}", finalstr);
    finalstr
}

// fn main() {
// 	testing();
// }

fn testing() {
	let mut teststr = "NOP";
	let mut testres = assemble(teststr.to_string());
	assert_eq!(testres, "0000000000000000".to_string());

	teststr = "; comment";
	testres = assemble(teststr.to_string());
	assert_eq!(testres, "NotInst".to_string());

	teststr = "LD R1, R2, 0x12; comment";
	testres = assemble(teststr.to_string());
	assert_eq!(testres, "0010001001010010".to_string());

	teststr = "ST R4, R11, 0x07";
	testres = assemble(teststr.to_string());
	assert_eq!(testres, "0100100101100111".to_string());

	teststr = "LIL R5, 0x27";
	testres = assemble(teststr.to_string());
	assert_eq!(testres, "1000101000100111".to_string());

	teststr = "LIH R5, 0xFC";
	testres = assemble(teststr.to_string());
	assert_eq!(testres, "1000101111111100".to_string());

	teststr = "ADD R5, R1";
	testres = assemble(teststr.to_string());
	assert_eq!(testres, "1010101000100000".to_string());

	teststr = "RRA R15, R8";
	testres = assemble(teststr.to_string());
	assert_eq!(testres, "1011111100001110".to_string());

	teststr = "JMP R15, 0x18";
	testres = assemble(teststr.to_string());
	assert_eq!(testres, "1101111000011000".to_string());

	teststr = "BN 0x58";
	testres = assemble(teststr.to_string());
	assert_eq!(testres, "1110010001011000".to_string());

}

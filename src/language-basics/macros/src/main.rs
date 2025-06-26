#![recursion_limit = "2048"]

macro_rules! brain_fudge {
    ($($token:tt)+) => {
        {
            let mut data = vec![0u8];
            let mut pointer = 0_usize;
            let mut output = Vec::new();

            brain_fudge_helper!(data, pointer, output, $($token)+);

            output.into_iter().map(char::from).collect::<String>()
        }
    };
}

macro_rules! brain_fudge_helper {
    ($data:ident, $pointer:ident, $buffer:ident, + $($token:tt)*) => {
        $data[$pointer] = $data[$pointer].wrapping_add(1);
        brain_fudge_helper!($data, $pointer, $buffer, $($token)*);
    };
    ($data:ident, $pointer:ident, $buffer:ident, - $($token:tt)*) => {
        $data[$pointer] = $data[$pointer].wrapping_sub(1);
        brain_fudge_helper!($data, $pointer, $buffer, $($token)*);
    };
    ($data:ident, $pointer:ident, $buffer:ident, > $($token:tt)*) => {
        $pointer = $pointer.saturating_add(1);
        while $pointer >= $data.len() {
            $data.push(0);
        }
        brain_fudge_helper!($data, $pointer, $buffer, $($token)*);
    };
    ($data:ident, $pointer:ident, $buffer:ident, < $($token:tt)*) => {
        $pointer = $pointer.saturating_sub(1);
        brain_fudge_helper!($data, $pointer, $buffer, $($token)*);
    };
    ($data:ident, $pointer:ident, $buffer:ident, . $($token:tt)*) => {
        $buffer.push($data[$pointer]);
        brain_fudge_helper!($data, $pointer, $buffer, $($token)*);
    };
    ($data:ident, $pointer:ident, $buffer:ident, [$($loop_statement:tt)+] $($token:tt)*) => {
        while $data[$pointer] != 0 {
            brain_fudge_helper!($data, $pointer, $buffer, $($loop_statement)+);
        }
        brain_fudge_helper!($data, $pointer, $buffer, $($token)*);
    };
    ($data:ident, $pointer:ident, $buffer:ident, ) => {};
    // Special "token" cases
    ($data:ident, $pointer:ident, $buffer:ident, >> $($token:tt)*) => {
        $pointer = $pointer.saturating_add(2);
        while $pointer >= $data.len() {
            $data.push(0);
        }
        brain_fudge_helper!($data, $pointer, $buffer, $($token)*);
    };
    ($data:ident, $pointer:ident, $buffer:ident, << $($token:tt)*) => {
        $pointer = $pointer.saturating_sub(2);
        brain_fudge_helper!($data, $pointer, $buffer, $($token)*);
    };
    ($data:ident, $pointer:ident, $buffer:ident, .. $($token:tt)*) => {
        $buffer.push($data[$pointer]);
        $buffer.push($data[$pointer]);
        brain_fudge_helper!($data, $pointer, $buffer, $($token)*);
    };
    ($data:ident, $pointer:ident, $buffer:ident, <- $($token:tt)*) => {
        $pointer = $pointer.saturating_sub(1);
        $data[$pointer] = $data[$pointer].wrapping_sub(1);
        brain_fudge_helper!($data, $pointer, $buffer, $($token)*);
    };
    ($data:ident, $pointer:ident, $buffer:ident, -> $($token:tt)*) => {
        $data[$pointer] = $data[$pointer].wrapping_sub(1);
        $pointer = $pointer.saturating_add(1);
        while $pointer >= $data.len() {
            $data.push(0);
        }
        brain_fudge_helper!($data, $pointer, $buffer, $($token)*);
    };
}

fn main() {
    assert_eq!(
        brain_fudge!(
            // H
            ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            // e
            >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            // l
            >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            // l
            >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            // o
            >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            //
            >++++++++++++++++++++++++++++++++.
            // W
            >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            // o
            >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            // r
            >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            // l
            >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            // d
            >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            // !
            >+++++++++++++++++++++++++++++++++.
            // \n
            >++++++++++.
        ),
        "Hello World!\n"
    );
    assert_eq!(
        brain_fudge!(++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.),
        "Hello World!\n"
    );
    println!("{}", brain_fudge!(
            -->+++>+>+>+>+++++>++>++>->+++>++>+>>>>>>>>>>>>>>>>->++++>>>>->+++>+++>+++>+++>+
            ++>+++>+>+>>>->->>++++>+>>>>->>++++>+>+>>->->++>++>++>++++>+>++>->++>++++>+>+>++
            >++>->->++>++>++++>+>+>>>>>->>->>++++>++>++>++++>>>>>->>>>>+++>->++++>->->->+++>
            >>+>+>+++>+>++++>>+++>->>>>>->>>++++>++>++>+>+++>->++++>>->->+++>+>+++>+>++++>>>
            +++>->++++>>->->++>++++>++>++++>>++[-[->>+[>]++[<]<]>>+[>]<--[++>++++>]+[<]<<++]
            >>>[>]++++>++++[--[+>+>++++<<[-->>--<<[->-<[--->>+<<[+>+++<[+>>++<<]]]]]]>+++[>+
            ++++++++++++++<-]>--.<<<]
        )
    );
    println!("{}", brain_fudge!(
            // Calculate the value 256 and test if it's zero
            // If the interpreter errors on overflow this is where it'll happen
            ++++++++[>++++++++<-]>[<++++>-]
            +<[>-<
                // Not zero so multiply by 256 again to get 65536
                [>++++<-]>[<++++++++>-]<[>++++++++<-]
                +>[>
                    // Print "32"
                    ++++++++++[>+++++<-]>+.-.[-]<
                <[-]<->] <[>>
                    // Print "16"
                    +++++++[>+++++++<-]>.+++++.[-]<
            <<-]] >[>
                // Print "8"
                ++++++++[>+++++++<-]>.[-]<
            <-]<
            // Print " bit cells\n"
            +++++++++++[>+++>+++++++++>+++++++++>+<<<<-]>-.>-.+++++++.+++++++++++.<.
            >>.++.+++++++..<-.>>-.
            // Clean up used cells
            [[-]<]
        )
    );
}

#[cfg(test)]
mod test {

    #[test]
    fn while_let_test() {
        let mut s = Vec::new();
        s.push(1);
        s.push(2);
        s.push(3);

        while let Some(top) = s.pop() {
            println!("{}", top);
        }
    }

    #[test]
    fn covered_matching_test() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    #[test]
    fn conditioned_matching_test() {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    #[test]
    fn ranged_matching_test() {
        let x = 3;
        match x {
            1..=4 => println!("1..=4"),
            5 => println!("5"),
            _ => println!("something else")
        }
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    #[test]
    fn destructured_enum_test() {
        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x, y
                );
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            ),
        }
    }

    #[test]
    fn matching_guard() {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }
}
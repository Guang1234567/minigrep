use std::fmt;
use std::io::{self, Write};


#[derive(Debug)]
struct Vector2D {
    x: isize,
    y: isize,
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Different traits allow different forms of output of a type. The meaning
// of this format is to print the magnitude of a vector.
impl fmt::Binary for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let magnitude = (self.x * self.x + self.y * self.y) as f64;
        let magnitude = magnitude.sqrt();

        // Respect the formatting flags by using the helper method
        // `pad_integral` on the Formatter object. See the method
        // documentation for details, and the function `pad` can be used
        // to pad strings.
        let decimals = f.precision().unwrap_or(3);
        let string = format!("{:.*}", decimals, magnitude);
        f.pad_integral(true, "", &string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, 2);

        // 通常情况下，`{}` 会被任意变量内容所替换。
        // 变量内容会转化成字符串。
        println!("{} days", 31);

        // 不加后缀的话，31 就自动成为 i32 类型。
        // 你可以添加后缀来改变 31 的类型。

        // 用变量替换字符串有多种写法。
        // 比如可以使用位置参数。
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

        // 可以使用命名参数。
        println!("{subject} {verb} {object}",
                 object = "the lazy dog",
                 subject = "the quick brown fox",
                 verb = "jumps over");

        // 可以在 `:` 后面指定特殊的格式。
        println!("{} of {:b} people know binary, the other half don't", 1, 2);


        // 你可以按指定宽度来右对齐文本。
        // 下面语句输出 "     1"，5 个空格后面连着 1。
        println!("{number:>width$}", number = 1, width = 6);

        // 你可以在数字左边补 0。下面语句输出 "000001"。
        println!("{number:>0width$}", number = 1, width = 6);

        // 取小数点后几位, 同时四舍五入
        let formatted_number = format!("{:.*}", 5, 1.234567);
        assert_eq!("1.23457", formatted_number);
        let formatted_number = format!("{:.*}", 3, 1.234567);
        assert_eq!("1.235", formatted_number);
        let formatted_number = format!("{:.*}", 2, 1.234567);
        assert_eq!("1.23", formatted_number);

        let myvector = Vector2D { x: 3, y: 4 };
        println!("{}", myvector);       // => "(3, 4)"
        println!("{:?}", myvector);     // => "Vector2D {x: 3, y:4}"
        println!("{:10.4b}", myvector); // => "     5.000"


        let mut some_writer = io::stdout();
        writeln!(&mut some_writer, "{} - {}", 123, 789);


        // Hello {arg 0 ("x")} is {arg 1 (0.01) with precision specified inline (5)}
        println!("Hello {0} is {1:.5}", "x", 0.01);

// Hello {arg 1 ("x")} is {arg 2 (0.01) with precision specified in arg 0 (5)}
        println!("Hello {1} is {2:.0$}", 5, "x", 0.01);

// Hello {arg 0 ("x")} is {arg 2 (0.01) with precision specified in arg 1 (5)}
        println!("Hello {0} is {2:.1$}", "x", 5, 0.01);

// Hello {next arg ("x")} is {second of next two args (0.01) with precision
//                          specified in first of next two args (5)}
        println!("Hello {} is {:.*}",    "x", 5, 0.01);

// Hello {next arg ("x")} is {arg 2 (0.01) with precision
//                          specified in its predecessor (5)}
        println!("Hello {} is {2:.*}",   "x", 5, 0.01);

// Hello {next arg ("x")} is {arg "number" (0.01) with precision specified
//                          in arg "prec" (5)}
        println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);
    }
}
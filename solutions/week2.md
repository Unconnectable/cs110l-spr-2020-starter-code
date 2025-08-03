## Part1

### 引用的一些问题

**Example 1**

```rust
fn main() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    s = String::from("goodbye");
    println!("{}", ref3.to_uppercase());
}
```

问题:`ref1/2/3`对 s 进行了引用,但是第六行对 s 进行了修改,这回导致悬空引用,在 rust 中不允许

修改方法:把 ref1/2/3 的作用域和 s 分开,这样修改 s 不会影响到引用

```rust
fn main() {
    let mut s = String::from("hello");
    {
        let ref1 = &s;
        let ref2 = &ref1;
        let ref3 = &ref2;
        println!("{}", ref3.to_uppercase());
    }
    s = String::from("goodbye");
    println!("{}", s);
}
```

**Example 2**

```rust
fn drip_drop() -> &String {
    let s = String::from("hello world!");
    return &s;
}
```

返回了对`String`类型的引用,在函数执行完成之后,s 会被销毁(drop),返回引用会导致悬垂引用,这是不允许的

**修改方法**

1.修改为返回`String`类型

```rust
fn drip_drop() -> String {
    String::from("hello world!")
}
```

2.添加生命周期,参数和返回一样的生命周期

```rust
fn drip_drop<'a>(s: &'a str) -> &'a str {
    s
}
```

**Example 3**

```rust
fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: String = v[0];
    println!("{}", s2);
}
```

问题:` let s2: String = v[0];`

此时的`s1`被移动的`v`中,s1 不可使用,而且 v[0]的类型是一个 String,他只能被移动,不能被复制 ,也就是第 5 行的复制是不被允许的

**修改方法**

```rust
//变为引用
let s2: &String = &v[0];

println!("{}", s2); //会自动把 &String 变成 &str类型

//显式clone数据
let s2: String = v[0].clone();
```

Rust 有一条核心的**借用规则**:

1. **在任意给定时间,您只能拥有:**
   - **一个可变引用** (mut borrow),或者
   - **任意数量的不可变引用** (immutable borrow).

也就是只能像下面这样

这是个很复杂的事情,暂时忽略

---

## Part2

### 1

`cargo test test_read_file_lines`

```rust
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    //let file = File::open(filename).unwrap();
    let path = File::open(filename)?;
    let mut lines = Vec::new();
    for line in io::BufReader::new(path).lines() {
        let line_str = line?;
        lines.push(line_str);
    }
    Ok(lines)
}

```

### 2

`cargo test test_grid -- --nocapture`

```rust
pub fn get(&self, row: usize, col: usize) -> Option<usize> {
    if row >= self.num_rows || col >= self.num_cols {
        return None;
    }
    let position = row * self.num_cols + col;
    if position >= self.elems.len() {
        return None;
    }
    Some(self.elems[position])
}

pub fn set(&mut self, row: usize, col: usize, val: usize) -> Result<(), &'static str> {
    if row >= self.num_rows || col >= self.num_cols {
        return Err("row or col Wrong");
    }
    let position = row * self.num_cols + col;
    if position >= self.elems.len() {
        return Err("🌎 position out of bounds");
    } else {
        self.elems[position] = val;
        Ok(())
    }
}

```

`cargo test test_lcs -- --nocapture`

要点:注意`get`和`set`的返回类型,不是 c 语言的 void,这里是`Option`和`Result`,保证无误的情况下需要通过`.unwrap()`解包处理

---

### 3

```rust
fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    let m = seq1.len();
    let n = seq2.len();
    let mut lcs_table = Grid::new(m + 1, n + 1);
    for i in 0..m + 1 {
        lcs_table.set(i, 0, 0).unwrap();
    }
    for j in 0..n + 1 {
        lcs_table.set(0, j, 0).unwrap();
    }

    for i in 0..m {
        for j in 0..n {
            if seq1[i] == seq2[j] {
                let val = lcs_table.get(i, j).unwrap() + 1;
                lcs_table.set(i + 1, j + 1, val);
            } else {
                let val1 = lcs_table.get(i + 1, j).unwrap();
                let val2 = lcs_table.get(i, j + 1).unwrap();
                lcs_table.set(i + 1, j + 1, max(val1, val2)).unwrap();
            }
        }
    }
    lcs_table
}
```

---

### 4

`print_diff`
为什么递归写法要先递归再输出?  
因为递归的起点是文本的末尾,从最后一行开始,如果此时输出 得到的结果是倒序 .

```sh
cargo run simple-a.txt simple-b.txt

#output
  e
> added
  d
> added
  c
  b
> added
  a
```

先递归再输出会再递归到第一行的时候进行输出,接着是第二行第三行直到最后一行,这才是正确的顺序  
循环写法同样,按照顺序收集的是从末尾到开头的输出,使用`rev`取反就行

**递归写法如下**

```rust
fn print_diff(
    lcs_table: &Grid,
    lines1: &Vec<String>,
    lines2: &Vec<String>,
    mut i: usize,
    mut j: usize
) {
    if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
        print_diff(lcs_table, lines1, lines2, i - 1, j - 1);
        println!("  {}", lines1[i - 1]);
    } else if
        j > 0 &&
        (i == 0 || lcs_table.get(i, j - 1).unwrap() >= lcs_table.get(i - 1, j).unwrap())
    {
        print_diff(lcs_table, lines1, lines2, i, j - 1);
        println!("> {}", lines2[j - 1]);
    } else if
        i > 0 &&
        (j == 0 || lcs_table.get(i, j - 1).unwrap() < lcs_table.get(i - 1, j).unwrap())
    {
        print_diff(lcs_table, lines1, lines2, i - 1, j);
        println!("< {}", lines1[i - 1]);
    } else {
        println!(" ");
    }
}
```

**循环写法如下**

```rust
// 这里的 i j 需要被修改 于是是mut类型
fn print_diff(
    lcs_table: &Grid,
    lines1: &Vec<String>,
    lines2: &Vec<String>,
    mut i: usize,
    mut j: usize
) {
    let mut ops = Vec::new();
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
            ops.push(format!("  {}", lines1[i - 1]));
            i -= 1;
            j -= 1;
        } else if
            j > 0 &&
            (i == 0 || lcs_table.get(i, j - 1).unwrap() >= lcs_table.get(i - 1, j).unwrap())
        {
            ops.push(format!("> {}", lines2[j - 1]));
            j -= 1;
        } else if
            i > 0 &&
            (j == 0 || lcs_table.get(i, j - 1).unwrap() < lcs_table.get(i - 1, j).unwrap())
        {
            ops.push(format!("> {}", lines1[i - 1]));
            i -= 1;
        }
    }

    for line in ops.iter().rev() {
        println!("{}", line);
    }
}
```

### 5

补充`main`函数  
注意的问题:`print_diff`的参数应该是序列的长度大小,而不是直接`let (m, n) = lca_table.size();`,因为`lcs`初始化将大小增加了 1,这样会直接遍历到边界导致错误

`cargo run handout-a.txt handout-b.txt`

`cargo run simple-a.txt simple-b.txt`

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let lines1_result = read_file_lines(filename1).expect("Failed to read file1");
    let lines2_result = read_file_lines(filename2).expect("Failed to read file2");
    let lcs_table = lcs(&lines1_result, &lines2_result);
    let m = lines1_result.len();
    let n = lines2_result.len();
    print_diff(&lcs_table, &lines1_result, &lines2_result, m, n);
}
```

---

### 6 rwc 写法

`cargo run Cargo.toml`

`cargo run Cargo.lock`  
注意需要在和文本同一文件夹下运行,也就是

```sh
week2/rwc$ ls
Cargo.lock  Cargo.toml  src  target
```

```rust
// cs110l-spr-2020-starter-code/week2/rwc/src/main.rs
use std::env;
use std::process;
use std::fs::File;
use std::io::{ self, BufRead };
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];

    let mut lines_count = 0;
    let mut words_count = 0;
    let mut chars_count = 0;

    let file = match File::open(filename) {
        Ok(file_) => file_,
        Err(_e) => {
            //编译器提示修改变量名称
            println!("File has Error");
            process::exit(1);
        }
    };
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        // 直接line? 会得到类型错误
        let line_str = line.expect("line error");
        lines_count += 1;
        chars_count += line_str.len();
        words_count += line_str.split_whitespace().count();
    }
    println!("lines: {}", lines_count);
    println!("words: {}", words_count);
    println!("chars: {}", chars_count);
    // Your code here :)
}
```

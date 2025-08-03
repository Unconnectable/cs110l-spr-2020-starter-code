## 1

### Milestone 1: 查找目标进程

- **`ps_utils::get_target()`**: 该函数接收一个字符串参数(进程名或 PID),并尝试在系统中找到对应的进程.其返回类型为 `Result<Option<Process>, Error>`,这意味着它可能成功(返回 `Ok`)或失败(返回 `Err`),如果成功,返回的结果可能找到进程(`Some(Process)`)或未找到(`None`).

**`main` 函数逻辑** 在 `main` 函数中,我们解析命令行参数,然后调用 `ps_utils::get_target()` 函数.

- 如果找到目标进程(`Some(proc)`),程序会打印该进程的信息,并进一步调用 `ps_utils::get_child_processes()` 来获取并打印其所有子进程的信息.
- 如果未找到目标进程(`None`),程序会打印一条错误消息并以退出码 1 退出.
- 如果 `get_target` 函数本身返回 `Err`,`expect()` 会导致程序崩溃并打印错误信息.

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <name or pid of target>", args[0]);
        std::process::exit(1);
    }
    let target = &args[1];

    let green = "\x1b[32m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";
    let ret = ps_utils::get_target(&target).expect("{green}Milestone1 :get_target ERROR! {reset}");

    match ret {
        Some(proc) => {
            println!("{green} Success find proc {reset}");
            proc.print();
            for child in ps_utils
                ::get_child_processes(proc.pid)
                .expect("failed to get child processes")
                .iter() {
                println!("{green} Success find child proc {reset}");
                child.print();
            }
        }
        None => {
            println!("{green}Target {red}{}{green} not match any running PIDs or executables{reset}", target);
            std::process::exit(1);
        }
    }
}
```

---

### Milestone 2: 实现 `Process::print` 函数

**`Process::print` 函数逻辑** 该方法首先打印进程的基本信息(命令、PID、PPID).然后,它会尝试调用 `self.list_open_files()` 方法来获取所有打开的文件描述符列表.

- 如果获取失败(`None`),则打印一条警告信息.
- 如果成功(`Some(open_files)`),它会遍历这个文件列表,并为每个文件描述符(`fd`)及其对应的文件信息(`file`)打印一行格式化好的输出,包括文件描述符编号、访问模式、光标位置和文件名称.

```rust
impl Process {
    pub fn new(pid: usize, ppid: usize, command: String) -> Process {
        Process { pid, ppid, command }
    }

    pub fn print(&self) {
        println!(
            "============== {} (pid {}, ppid {}) =============",
            self.command,
            self.pid,
            self.ppid
        );
        let red = "\x1b[31m";
        let reset = "\x1b[0m";
        match self.list_open_files() {
            None =>
                println!(
                    "{red} Warning: could not inspect file descriptors for this process! It might have exited just as we were about to look at its fd table, or it might have exited a while ago and is waiting for the parent to reap it.{reset}"
                ),
            Some(open_files) => {
                for (fd, file) in open_files {
                    println!(
                        "{:<5} {:<20} cursor: {:<5} {}",
                        fd,
                        format!("({})", file.access_mode),
                        file.cursor,
                        file.colorized_name()
                    );
                }
            }
        }
    }
}
```

---

### Milestone 3: 实现 `Process::list_fds` 函数

实现一个函数,用于列出指定进程的所有文件描述符.

**`Process::list_fds` 函数逻辑** 该函数通过访问 `/proc/{pid}/fd` 目录来工作.

- `format!` 宏构建出 `/proc/{pid}/fd` 的路径.

  使用 `fs::read_dir()` 读取该目录,并使用 `.ok()?` 进行错误处理,如果失败则提前返回 `None`.

- 遍历目录中的每一个条目.在循环内部,对每个条目也使用 `.ok()?` 进行错误处理.

- 提取出每个文件描述符的名称(一个数字字符串),将其转换为 `usize` 类型,并存入一个 `Vec<usize>` 中.

- 将这个 `Vec` 用 `Some` 包裹并返回.

**代码示例**

```rust
pub fn list_fds(&self) -> Option<Vec<usize>> {
    let dir = format!("/proc/{}/fd", self.pid);
    let mut fds = Vec::<usize>::new();
    for entry in fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        fds.push(entry.file_name().into_string().unwrap().parse().unwrap());
    }
    return Some(fds);
}
```

---

### Milestone 4: 实现 `OpenFile::from_fd` 函数

本阶段的目标是实现一个函数,根据进程 ID 和文件描述符获取一个 `OpenFile` 实例,该实例包含了文件的名称和元数据.

**`OpenFile::from_fd` 函数逻辑** 该函数分为两大部分:

1. **文件路径和名称的获取**
   - 构建 `/proc/{pid}/fd/{fd}` 路径.
   - 使用 `fs::read_link` 读取该符号链接,获取实际文件路径.
   - 将路径转换为字符串切片 (`&str`).
   - 调用 `OpenFile::path_to_name` 方法来提取出可读的文件名.
2. **文件元数据的获取**
   - 构建 `/proc/{pid}/fdinfo/{fd}` 路径.
   - 使用 `fs::read_to_string` 读取其内容.
   - 调用 `OpenFile::parse_cursor` 方法从内容中解析出文件光标位置.
   - 调用 `OpenFile::parse_access_mode` 方法从内容中解析出文件的访问模式.

**最终返回**

- 如果上述所有步骤都成功,函数会使用收集到的 `name`、`cursor` 和 `mode` 创建一个新的 `OpenFile` 实例,并将其包裹在 `Some` 中返回.任何一步失败,`?` 运算符都会让函数提前返回 `None`.

**代码示例**

```rust
pub fn from_fd(pid: usize, fd: usize) -> Option<OpenFile> {
    // 找到文件描述符 fd 对应的真实文件路径,并从中提取出文件名称
    let fd_path = format!("/proc/{}/fd/{}", pid, fd);
    let name_path = fs::read_link(fd_path).ok()?;
    let name_str = name_path.to_str()?;
    let name = OpenFile::path_to_name(name_str);

    // 读取文件描述符的元数据,比如当前的文件读写位置和访问模式
    let fdinfo = format!("/proc/{}/fdinfo/{}", pid, fd);
    let content = fs::read_to_string(fdinfo).ok()?;
    let cursor = OpenFile::parse_cursor(&content)?;
    let mode = OpenFile::parse_access_mode(&content)?;
    Some(OpenFile::new(name, cursor, mode))
}
```

### 最终测试命令 `cargo test`

输出如下

```sh
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s
```

## 2

### `main.rs`的案例

```rust
use linked_list::LinkedList;
pub mod linked_list;
use crate::linked_list::ComputeNorm;
fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
    let green = "\x1b[32m";
    let reset = "\x1b[0m";
    println!("\n{green}String Type LinkList{reset}:");
    let mut list: LinkedList<String> = LinkedList::new();
    println!("List get_size() is: {}", list.get_size());
    assert!(list.is_empty());
    list.push_front("a".to_string());
    list.push_front("bb".to_string());
    list.push_front("ccc".to_string());
    list.push_front("dddd".to_string());
    println!("list is: {}", list);
    let mut list_clone = list.clone();
    println!("{green}list_clone: {}{reset}", list_clone);

    list_clone.push_front("eeeee".to_string());
    list_clone.push_front("f".to_string());

    //test Eq
    list.push_front("eeeee".to_string());
    list.push_front("f".to_string());

    println!("list = list_clone: {}", list == list_clone);

    //Norm
    println!("\n{green}Calculate Nrom{reset}");
    let mut f64_list: LinkedList<f64> = LinkedList::new();
    f64_list.push_front(300.0);
    f64_list.push_front(400.0);
    println!("{}", f64_list.compute_norm());
}
```

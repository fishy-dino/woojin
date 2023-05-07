# woojin
This is Woojinlang interpreter written in Rust(Current v0.1.0).<br/>
The Woojinlang project is a project that was started as a joke to tease my friend. Do not use this language in your projects.<br/>
The **0.x.x** version is an unofficial version. Current grammar may not be compatible with the official version (>=1.x.x)<br/>

Click here to read the [manual](https://teamfishydino.github.io/fishydocs/).

[node-woojin](https://github.com/minjeadev/node-woojin), the only existing woojinlang interpreter, is no longer being developed. Also, there are so many problems with code in node-woojin that we recommend that you no longer use it. **Use this safer, more improved program instead of node-woojin**.

Our developers like Rust, but we are not good at it. In other words, there may be serious bugs in this code or unnecessary code. We are ready to accept any advice from you. If you have any comments on Woojinlang's grammar, or your opinions of Woojin, or the our code etc, please feel free to contact us(teamfishydino@gmail.com)

Thank you so much for using our program.
## How To Install
* **Install Rust first** (if you have already installed the latest version of Rust on your computer, you may skip this process)<br/>https://www.rust-lang.org/tools/install
* **Download woojin with cargo**
```shell
cargo install woojin --all-features
```
## Example
Here's an example for woojin v0.1.0.<br/>
Do you want more example? [Click Here!](https://github.com/teamfishydino/woojin-example/tree/main/example)

First, Create a file named main.wj.
```woojin
println "My First Woojinlang Program!";
let name = input "Hello, What is your Name? ";
println "Hello, "+$name+". Nice to meet you!";
yee 0;
```
And use the command below to run the woojin file
```shell
woojin main.wj
```
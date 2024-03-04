use navm::{
    operation,
    vm::{self, *},
};

struct EchoVM;

impl vm::VM for EchoVM {
    fn input_cmd(&mut self, cmd: navm::nair::Cmd) {
        println!("输入的指令：{cmd:?}");
    }

    fn fetch_output(&mut self) -> Option<vm::Output> {
        println!("似乎没有输出");
        None
    }

    fn add_output_listener<Listener>(&mut self, _listener: Listener)
    where
        Listener: FnMut(vm::Output) -> Option<vm::Output>,
    {
        println!("尝试添加输出侦听器！")
    }

    fn store_output(&mut self, output: vm::Output) {
        println!("存入输出：{output:?}")
    }

    fn iter_output_listeners<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a mut dyn FnMut(vm::Output) -> Option<vm::Output>> + 'a> {
        println!("似乎没有输出侦听器");
        Box::new(vec![].into_iter())
    }
}

fn main() {
    // 创建虚拟机
    let mut vm = EchoVM;

    // 输入指令
    let c1 = navm::nair::Cmd::SAV {
        target: "test".to_string(),
        path: "test".to_string(),
    };
    let c2 = navm::nair::Cmd::from_str_params("CUS", "这是一条自定义指令");
    vm.input_cmd(c1);
    vm.input_cmd(c2);

    // 输出
    let o = vm::Output::ANSWER { narsese: "<{SELF} --> [good]>".into() };
    let op = operation!("say" => "{SELF}" "hello, world");
    vm.on_output(o);
    vm.on_output(vm::Output::EXE(op))
}

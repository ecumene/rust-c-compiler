use crate::parse::*;

pub trait Compile {
    fn compile(self) -> String;
}

impl Compile for Const {
    fn compile(self) -> String {
        self.0.to_string()
    }
}

impl Compile for Return {
    fn compile(self) -> String {
        format!(
            "\
movl    ${}, %eax
    ret",
            self.0.compile(),
        )
    }
}

impl Compile for Fun {
    fn compile(self) -> String {
        format!(
            "
    .globl {0}
{0}:
    {1}",
            self.0,
            self.1.compile()
        )
    }
}

impl Compile for Prog {
    fn compile(self) -> String {
        self.0.compile()
    }
}

impl Compile for &str {
    fn compile(self) -> String {
        self.parse::<Prog>().unwrap().compile()
    }
}

impl Compile for String {
    fn compile(self) -> String {
        self.parse::<Prog>().unwrap().compile()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_lexes_sample_input() {
        let input = "\
int main() {
    return 2;
}";
        let output = "
    .globl main
main:
    movl    $2, %eax
    ret";
        assert_eq!(input.compile(), output);
    }
}

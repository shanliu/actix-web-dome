#[test]
fn test1(){
    use std::ops::Add;
    #[derive(Debug,Copy, Clone)]
    struct A1{
        value:i32
    }
    impl Add<i32> for A1{
        type Output = i32;
        fn add(mut self, rhs: i32) -> Self::Output {
            self.value=self.value+rhs;
            self.value
        }
    }
    impl Add for A1{
        type Output = Self;
        fn add(mut self, rhs: Self) -> Self::Output {
            self.value=self.value+rhs.value;
            self
        }
    }
    let a=A1{value:1};
    let c=a+1;
    let d=a+a;
    println!("{:?}-{:?}",c,d);
}

use std::io::{Error, ErrorKind};

#[test]
fn test3(){




    //复合类型中泛型限定方法增加
    trait Mlimit{}
    //impl<T> Mlimit for T{} //最顶层泛型 全局泛型实现
    struct Mytest<T1> {a:T1}
    //impl<T1:Mlimit> Mlimit for Mytest<T1> {} //类型实现 trait
    //impl<T3> Mytest<T3>{}//类型增加方法
    impl<T3:Mlimit> Mytest<T3>//给类型内部泛型限定下增加该类型方法
    {
        fn fn1(self) {}
    }
    impl Mlimit for i32{}
    (Mytest{a:1}).fn1();
    //struct MYTYPE<SUBTYPE>
    // SUBTYPE:LIMIT_TRAIT
    //     实现 LIMIT_TRAIT 一批子类型增加方法实现
    //     SUBTYPE指定类型实现
}


#[test]
fn test2(){
    fn tt()->std::io::Result<()>{
        let error = std::io::Error::from_raw_os_error(10022);
        let stderr=std::io::Result::Err(error)?;
        Ok(())
    }
    let b=tt();
    b.map_err(|e|{
        let a=e.kind();
        println!("{:?}",a);
    });
}

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



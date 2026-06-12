fn main() {}

trait State {
    fn verify(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str>;
    fn suspend(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str>;
    fn unsuspend(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str>;
    fn delete(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str>;
}

struct UnverifiedState;
struct ActiveState;
struct SuspendedState;
// terminal state
struct DeletedState;

impl State for UnverifiedState {
    fn verify(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        println!("验证完成，即将激活用户");
        Ok(Box::new(ActiveState))
    }

    fn suspend(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        println!("该用户还不是一个正式用户，无法停用");
        Err("该用户还不是一个正式用户，无法停用")
    }

    fn unsuspend(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        println!("该用户还是不是一个正式用户，无法解除停用");
        Err("该用户还是不是一个正式用户，无法解除停用")
    }

    fn delete(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        println!("该用户未验证，执行注销");
        Ok(Box::new(DeletedState))
    }
}

impl State for ActiveState {
    fn verify(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        Err("it is already active")
    }

    fn suspend(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        Ok(Box::new(SuspendedState))
    }

    fn unsuspend(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        Err("it is not a suspended user now")
    }

    fn delete(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        Ok(Box::new(DeletedState))
    }
}

impl State for SuspendedState {
    fn verify(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        todo!()
    }

    fn suspend(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        todo!()
    }

    fn unsuspend(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        todo!()
    }

    fn delete(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        todo!()
    }
}

impl State for DeletedState {
    fn verify(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        todo!()
    }

    fn suspend(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        todo!()
    }

    fn unsuspend(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        todo!()
    }

    fn delete(self: Box<Self>, ctx: ()) -> Result<Box<dyn State>, &'static str> {
        todo!()
    }
}

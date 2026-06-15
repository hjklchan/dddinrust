use chrono::Utc;

fn main() {
    let mut user = User::new("lucas".to_owned(), Box::new(UnverifiedState));
}

struct User {
    name: String,
    state: Box<dyn State>,

    verified_at: Option<chrono::DateTime<Utc>>,
    deleted_at: Option<chrono::DateTime<Utc>>,
}

impl User {
    pub fn new(name: String, state: Box<dyn State>) -> User {
        User {
            name,
            state,
            verified_at: None,
            deleted_at: None,
        }
    }

    pub fn verify(&mut self) -> Result<(), &'static str> {
        Ok(())
    }
}

trait State {
    fn verify(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str>;
    fn suspend(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str>;
    fn unsuspend(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str>;
    fn delete(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str>;
}

struct UnverifiedState;
struct ActiveState;
struct SuspendedState;
// terminal state
struct DeletedState;

impl State for UnverifiedState {
    fn verify(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        ctx.verified_at = Utc::now();

        println!("验证完成，激活用户");
        Ok(Box::new(ActiveState))
    }

    fn suspend(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        println!("该用户还不是一个正式用户，无法停用");
        Err("该用户还不是一个正式用户，无法停用")
    }

    fn unsuspend(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        println!("该用户还是不是一个正式用户，无法解除停用");
        Err("该用户还是不是一个正式用户，无法解除停用")
    }

    fn delete(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        println!("该用户未验证，执行注销");
        Ok(Box::new(DeletedState))
    }
}

impl State for ActiveState {
    fn verify(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        let msg = "用户yibeijih";
        println!("{}", msg);
        Err(msg)
    }

    fn suspend(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        let msg = "用户已被冻结";
        println!("{}", msg);
        Ok(Box::new(SuspendedState))
    }

    fn unsuspend(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        let msg = "当前用户不是冻结状态";
        println!("{}", msg);
        Err(msg)
    }

    fn delete(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        let msg = "无法直接删除用户";
        println!("{}", msg);
        Err(msg)
    }
}

impl State for SuspendedState {
    fn verify(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        println!("被冻结的用户无法执行验证");
        Err("被冻结的用户无法执行验证")
    }

    fn suspend(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        let msg = "该用户已经处于冻结状态";
        println!("{}", msg);
        Err(msg)
    }

    fn unsuspend(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        let msg = "用户恢复正常状态";
        println!("{}", msg);
        Ok(Box::new(ActiveState))
    }

    fn delete(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        let msg = "冻结的用户已被删除";
        println!("{}", msg);
        Ok(Box::new(DeletedState))
    }
}

impl State for DeletedState {
    fn verify(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        Err(self.reject())
    }

    fn suspend(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        Err(self.reject())
    }

    fn unsuspend(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        Err(self.reject())
    }

    fn delete(self: Box<Self>, ctx: &mut User) -> Result<Box<dyn State>, &'static str> {
        Err(self.reject())
    }
}

impl DeletedState {
    fn reject(&self) -> &'static str {
        let msg = "拒绝操作，用户已被删除";
        println!("{}", msg);
        msg
    }
}

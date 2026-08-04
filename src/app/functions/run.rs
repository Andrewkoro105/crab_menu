use std::pin::Pin;

pub trait Run: Send + Clone {
    fn run(&self) -> impl Future<Output = ()> + Send;

    fn close_id(&self) -> bool;
}

pub trait BoxedRun: Send {
    fn run<'future>(&'future self) -> Pin<Box<dyn Future<Output = ()> + Send + 'future>>;

    fn close_id(&self) -> bool;

    fn clone_box(&self) -> Box<dyn BoxedRun>;
}

impl<T: Run + 'static> BoxedRun for T {
    fn run<'future>(&'future self) -> Pin<Box<dyn Future<Output = ()> + Send + 'future>> {
        Box::pin(<Self as Run>::run(&self))
    }

    fn close_id(&self) -> bool {
        <Self as Run>::close_id(&self)
    }

    fn clone_box(&self) -> Box<dyn BoxedRun> {
        Box::new(self.clone()) as Box<dyn BoxedRun>
    }
}

impl BoxedRun for Box<dyn BoxedRun> {
    fn run<'future>(&'future self) -> Pin<Box<dyn Future<Output = ()> + Send + 'future>> {
        self.as_ref().run()
    }

    fn close_id(&self) -> bool {
        self.as_ref().close_id()
    }

    fn clone_box(&self) -> Box<dyn BoxedRun> {
        self.clone()
    }
}

impl Clone for Box<dyn BoxedRun> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}


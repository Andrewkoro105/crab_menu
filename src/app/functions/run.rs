use std::pin::Pin;

pub trait Run: Send + Sync + Clone {
    fn run(&self) -> impl Future<Output = ()> + Send;

    fn close_it(&self) -> bool;
}

pub trait BoxedRun: Send + Sync {
    fn run<'future>(&'future self) -> Pin<Box<dyn Future<Output = ()> + Send + 'future>>;

    fn close_it(&self) -> bool;

    fn clone_box(&self) -> Box<dyn BoxedRun>;
}

impl<T: Run + 'static> BoxedRun for T {
    fn run<'future>(&'future self) -> Pin<Box<dyn Future<Output = ()> + Send + 'future>> {
        Box::pin(<Self as Run>::run(&self))
    }

    fn close_it(&self) -> bool {
        <Self as Run>::close_it(&self)
    }

    fn clone_box(&self) -> Box<dyn BoxedRun> {
        Box::new(self.clone()) as Box<dyn BoxedRun>
    }
}

impl BoxedRun for Box<dyn BoxedRun> {
    fn run<'future>(&'future self) -> Pin<Box<dyn Future<Output = ()> + Send + 'future>> {
        self.as_ref().run()
    }

    fn close_it(&self) -> bool {
        self.as_ref().close_it()
    }

    fn clone_box(&self) -> Box<dyn BoxedRun> {
        self.as_ref().clone_box()
    }
}

impl Clone for Box<dyn BoxedRun> {
    fn clone(&self) -> Self {
        let result = self.clone_box();
        result
    }
}
